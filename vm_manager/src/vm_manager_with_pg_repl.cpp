#include <iostream>
#include <fstream>
#include <moe/log.hpp>
#include <moe/moe_typedef.hpp>
#include <moe/rt_check.hpp>
#include <moe/debugger.hpp>
#include <moe/arg_parser.hpp>

constexpr u32 MEM_SIZE = 1 << 16;
constexpr u32 PG_SIZE = 1 << 8;
u8 pm[MEM_SIZE];
u8 bs[MEM_SIZE];

void read_bs_from_bin_file(std::string_view binFilePath) {
	auto ifs = std::ifstream(binFilePath, std::ios::in | std::ios::binary);
	moe_assert(ifs, "Fail to read form bin.");
	for (u32 pa = 0; pa < MEM_SIZE; ++pa) {
		ifs.read(reinterpret_cast<char *>(bs + pa), 1);
	}
}

constexpr u32 PTE_NUMBER = 256;

struct PTE {
	bool valid = false;
	u32 frameNumber = 0;
} pte[PTE_NUMBER];


constexpr u32 TLB_NUMBER = 16;
struct TLB {
	u32 pageNumber = 0;
	u32 frameNumber = 0;
	bool valid = false;
} tlb[TLB_NUMBER];

//  frame occupied fifo
namespace fifo {
constexpr u32 FIFO_SZ = 128;
struct FifoNode {
	u32 pageNumber = 0;
	u32 frameNumber = 0;
} fifoNodes[FIFO_SZ];
u32 fifoTailIdx, fifoHeadIdx;

bool empty() {
	return fifoHeadIdx == fifoTailIdx;
}

bool full() {
	return fifoHeadIdx + FIFO_SZ == fifoTailIdx;
}

FifoNode & front() {
	moe_assert(!empty(), "fifo: empty");
	return fifoNodes[fifoHeadIdx % FIFO_SZ];
}

void pop() {
	++fifoHeadIdx;
}

void push(FifoNode && fifoNode) {
	moe_assert(!full(), "fifo: full");
	fifoNodes[fifoTailIdx % FIFO_SZ] = fifoNode;
	++fifoTailIdx;
}

u32 size() {
	return fifoTailIdx - fifoHeadIdx;
}
}

//  frame freed list
namespace free_frame_fifo {
constexpr u32 LST_SZ = 128;
struct LstNode {
	u32 frameNumber;
} lst[LST_SZ];
u32 lstTailIdx, lstHeadIdx;

bool empty() {
	return lstHeadIdx == lstTailIdx;
}

bool full() {
	return lstHeadIdx + LST_SZ == lstTailIdx;
}

LstNode & front() {
	moe_assert(!empty(), "lst: empty");
	return lst[lstHeadIdx % LST_SZ];
}

void pop() {
	++lstHeadIdx;
}

void push(LstNode && lstNode) {
	moe_assert(!full(), "lst: full");
	lst[lstTailIdx % LST_SZ] = lstNode;
	++lstTailIdx;
}

u32 size() {
	return lstTailIdx - lstHeadIdx;
}

void init() {
	for (auto i = u32(0); i < LST_SZ; ++i) {
		push(
			LstNode{
				.frameNumber=i,
			}
		);
	}
}

}

int main(int argc, char ** argv) {
	auto addressesFilePath = std::string();
	auto argParser = moe::ArgParser("vm_manager");
	argParser.add_func_to_handle_non_option_arg(
		[&addressesFilePath](std::string_view str) {
			addressesFilePath = str;
		}
	);
	argParser.parse(argc, argv);
	moe_assert(!addressesFilePath.empty());
	
	moe::register_std_log("log/std_log.txt");
	read_bs_from_bin_file("data/BACKING_STORE.bin");
	auto addressesStream = std::ifstream(addressesFilePath);
	auto ansOutStream = std::ofstream("data/my_ans_value.txt");
	
	auto totalAccessCnt = u32(0);
	auto pageFaultCnt = u32(0);
	auto replCnt = u32(0);
	auto tlbHitCnt = u32(0);
	
	free_frame_fifo::init();
	
	for (u32 va; addressesStream >> va;) {
		moe_slog_info(fifo::size(), free_frame_fifo::size());
		++totalAccessCnt;
		va = va & 0x0000FFFF;
		auto pgNumber = va >> 8;
		auto offset = va & 0xFF;
		auto tlbIdx = pgNumber % TLB_NUMBER;
		auto frameNumber = u32(0);
		
		if (tlb[tlbIdx].valid && tlb[tlbIdx].pageNumber == pgNumber) {  //  tlb hit
			frameNumber = tlb[tlbIdx].frameNumber;
			++tlbHitCnt;
		} else {    //  tlb not hit
			if (!pte[pgNumber].valid) { //  page fault for invalid page
				++pageFaultCnt;
				if (free_frame_fifo::empty()) {   //  free a frame
					++replCnt;
					auto oldFrame = fifo::front();
					fifo::pop();
					pte[oldFrame.pageNumber].valid = false;
					auto oldFrameTlbIdx = oldFrame.pageNumber & 0x3;
					if (tlb[oldFrameTlbIdx].valid &&
						tlb[oldFrameTlbIdx].pageNumber == oldFrame.pageNumber) {
						tlb[oldFrameTlbIdx].valid = false;
					}
					free_frame_fifo::push(
						free_frame_fifo::LstNode{
							.frameNumber=oldFrame.frameNumber
						}
					);
				}
				//  alloc frame from free frame list
				auto frame = free_frame_fifo::front();
				free_frame_fifo::pop();
				pte[pgNumber] = {
					.valid=true,
					.frameNumber=frame.frameNumber,
				};
				auto iFrame = pte[pgNumber].frameNumber;
				memmove(pm + (iFrame << 8), bs + (pgNumber << 8), PG_SIZE);
				fifo::push(
					fifo::FifoNode{
						.pageNumber=pgNumber,
						.frameNumber=iFrame,
					}
				);
			}
			//  fill tlb
			frameNumber = pte[pgNumber].frameNumber;
			tlb[tlbIdx] = {
				.pageNumber=pgNumber,
				.frameNumber=frameNumber,
				.valid=true
			};
		}
		auto pa = (frameNumber << 8) | offset;
//		ansOutStream << "Virtual address: " << va << " Physical address: " << pa
//		             << " Value: " << i32(i8(pm[pa])) << '\n';
		ansOutStream << i32(i8(pm[pa])) << '\n';
		moe_slog_info(va, pgNumber, offset, pa);
	}
	std::cout << "totalAccessCnt=" << totalAccessCnt << std::endl;
	std::cout << "pageFaultCnt=" << pageFaultCnt << std::endl;
	std::cout << "replCnt=" << replCnt << std::endl;
	std::cout << "tlbHitCnt=" << tlbHitCnt << std::endl;
	return 0;
}