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
//	auto ansOutStream = std::ofstream("data/my_ans.txt");
	auto ansOutStream = std::ofstream("data/my_ans_value.txt");
	
	auto totalAccessCnt = u32(0);
	auto pageFaultCnt = u32(0);
	auto tlbHitCnt = u32(0);
	for (u32 va, iFrame = 0; addressesStream >> va;) {
		++totalAccessCnt;
		va = va & 0x0000FFFF;
		auto pgNumber = va >> 8;
		auto offset = va & 0xFF;
		auto tlbIdx = pgNumber % TLB_NUMBER;
		auto frameNumber = u32(0);
		if (tlb[tlbIdx].valid && tlb[tlbIdx].pageNumber == pgNumber) {
			frameNumber = tlb[tlbIdx].frameNumber;
			++tlbHitCnt;
		} else {
			if (!pte[pgNumber].valid) {
				pte[pgNumber] = {
					.valid=true,
					.frameNumber=iFrame,
				};
				memmove(pm + (iFrame << 8), bs + (pgNumber << 8), PG_SIZE);
				++iFrame;
				++pageFaultCnt;
			}
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
	std::cout << "tlbHitCnt=" << tlbHitCnt << std::endl;
	return 0;
}
