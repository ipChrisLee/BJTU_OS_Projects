#include <iostream>
#include "moe/log.hpp"
#include <charconv>
#include "moe/arg_parser.hpp"
#include "moe/moe_typedef.hpp"
#include "moe/rt_check.hpp"
#include "moe/stl_pro.hpp"
#include <list>


u32 mxMem;

struct MemRegion {
	u32 le, ri;
	std::string p;
	
	[[nodiscard]] u32 size() const {
		return ri - le;
	}
	
	[[nodiscard]] bool freed() const {
		return p.empty();
	}
	
	friend std::ostream & operator<<(std::ostream & os, const MemRegion & memRegion) {
		if (memRegion.p.empty()) {
			os << "Addresses [" << memRegion.le << ":" << memRegion.ri << ") Unused"
			   << std::endl;
		} else {
			os << "Addresses [" << memRegion.le << ":" << memRegion.ri << ") Process "
			   << memRegion.p << std::endl;
		}
		return os;
	}
};

std::list<MemRegion> regions;

void
allocate_mem(std::list<MemRegion>::iterator it, u32 size, std::string_view processName) {
	auto newMem = MemRegion{
		.le=it->le,
		.ri=it->le + size,
		.p={processName.begin(), processName.end()},
	};
	regions.insert(it, std::move(newMem));
	if (it->le + size < it->ri) {
		it->le = it->le + size;
	} else if (it->le + size == it->ri) {
		regions.erase(it);
	} else {
		moe_panic("");
	}
}

void free_mem(std::list<MemRegion>::iterator it) {
	if (it != regions.begin()) {
		auto pre = std::prev(it);
		if (pre->ri == it->le && pre->freed()) {
			it->le = pre->le;
			regions.erase(pre);
		}
	}
	auto nxt = std::next(it);
	if (nxt != regions.end()) {
		if (nxt->le == it->ri && nxt->freed()) {
			it->ri = nxt->ri;
			regions.erase(nxt);
		}
	}
	it->p.clear();
}

bool rq_first_hit(u32 size, std::string_view processName) {
	auto it = regions.begin();
	while (it != regions.end()) {
		if (it->size() >= size && it->freed()) {
			break;
		}
		++it;
	}
	if (it == regions.end()) {
		return false;
	} else {
		allocate_mem(it, size, processName);
		return true;
	}
}

bool rq_best_hit(u32 size, std::string_view processName) {
	auto it = regions.begin();
	auto target = regions.end();
	while (it != regions.end()) {
		if (it->size() >= size && it->freed()) {
			if (target != regions.end()) {
				if (target->size() > it->size()) {
					target = it;
				}
			} else {
				target = it;
			};
		}
		++it;
	}
	if (target == regions.end()) {
		return false;
	} else {
		allocate_mem(target, size, processName);
		return true;
	}
}

bool rq_worst_hit(u32 size, std::string_view processName) {
	auto it = regions.begin();
	auto target = regions.end();
	while (it != regions.end()) {
		if (it->size() >= size && it->freed()) {
			if (target != regions.end()) {
				if (target->size() < it->size()) {
					target = it;
				}
			} else {
				target = it;
			};
		}
		++it;
	}
	if (target == regions.end()) {
		return false;
	} else {
		allocate_mem(target, size, processName);
		return true;
	}
}

void rl(std::string_view processName) {
	auto it = regions.begin();
	while (it != regions.end()) {
		if (it->p == processName) {
			free_mem(it);
		}
		++it;
	}
}

void cont() {
	auto addr = u32(0);
	auto newRegions = std::list<MemRegion>();
	for (const auto & region: regions) {
		if (!region.freed()) {
			newRegions.push_back(
				MemRegion{
					.le=addr,
					.ri=addr + region.size(),
					.p=region.p,
				}
			);
			addr = addr + region.size();
		}
	}
	if (addr < mxMem) {
		newRegions.push_back(
			MemRegion{
				.le=addr,
				.ri=mxMem,
				.p=""
			}
		);
	} else if (addr == mxMem) {
	} else {
		moe_panic("");
	}
	regions = newRegions;
}

void status() {
	for (const auto & region: regions) {
		std::cout << region;
	}
}

int main(int argc, char ** argv) {
	moe::register_std_log("log/std_log.txt");
	
	auto argParser = moe::ArgParser("cont_memory_allocation");
	argParser.add_func_to_handle_non_option_arg(
		[](std::string_view sv) {
			auto res = std::from_chars(sv.begin(), sv.end(), mxMem);
			moe_assert(res.ec == std::errc(), "Invalid input!");
		}
	);
	argParser.parse(argc, argv);
	moe_assert(mxMem > 0);
	regions.push_back(
		MemRegion{
			.le=0,
			.ri=mxMem,
			.p="",
		}
	);
	
	auto buffer = std::string();
	while (std::getline(std::cin, buffer)) {
		auto items = moe::split_string_on_char(buffer, " \n");
		moe_assert(!items.empty());
		if (items[0] == "RQ") {
			moe_assert(items.size() == 4);
			auto processName = items[1];
			auto size = u32(0);
			if (std::from_chars(
				items[2].c_str(), items[2].c_str() + items[2].size(), size
			).ec != std::errc()) {
				moe_panic("");
			}
			auto mode = items[3][0];
			auto res = bool(false);
			switch (mode) {
				case 'F': {
					res = rq_first_hit(size, processName);
					break;
				}
				case 'B': {
					res = rq_best_hit(size, processName);
					break;
				}
				case 'W': {
					res = rq_worst_hit(size, processName);
					break;
				}
				default:
					moe_panic("");
			}
			if (!res) { std::cout << "RQ: Failed" << std::endl; }
		} else if (items[0] == "RL") {
			moe_assert(items.size() == 2);
			auto processName = items[1];
			rl(processName);
		} else if (items[0] == "C") {
			moe_assert(items.size() == 1);
			cont();
		} else if (items[0] == "STAT") {
			moe_assert(items.size() == 2);
			auto comment = items[1];
			std::cout << comment << ": " << std::endl;
			status();
		}
	}
	
	return 0;
}
