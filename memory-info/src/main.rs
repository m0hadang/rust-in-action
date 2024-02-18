use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::memoryapi::VirtualQueryEx;
use winapi::um::processthreadsapi::{GetCurrentProcess, GetCurrentProcessId};
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::{HANDLE, MEMORY_BASIC_INFORMATION};

fn main() {
	let this_pid: DWORD = unsafe {
		GetCurrentProcessId()
	};
	let this_proc: HANDLE = unsafe {
		GetCurrentProcess()
	};
	let min_app_addr: LPVOID;
	let max_app_addr: LPVOID;
	let mut base_addr: LPVOID = unsafe { std::mem::zeroed() };
	let mut proc_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
	let mut mem_info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
	const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();
	unsafe {
		GetSystemInfo(&mut proc_info);
	}

	min_app_addr = proc_info.lpMinimumApplicationAddress;
	max_app_addr = proc_info.lpMaximumApplicationAddress;

	print_proc_info1(this_pid, this_proc);
	print_proc_info2(&proc_info);
	print_min_max_addr(min_app_addr, max_app_addr);
	loop {
		let rc: SIZE_T = unsafe {
			VirtualQueryEx(
				this_proc,
				base_addr,
				&mut mem_info,
				MEMINFO_SIZE as SIZE_T
			)
		};
		if rc == 0 {
			break;
		}
		println!("-------------------------");
		print_mem_info(&mem_info);
		base_addr = (mem_info.BaseAddress as usize + mem_info.RegionSize) as LPVOID
	}
}



fn print_proc_info1(this_pid: DWORD, this_proc: HANDLE) {
	println!("{:?} @ {:p}", this_pid, this_proc);
}

fn print_proc_info2(proc_info: &SYSTEM_INFO) {
	unsafe {
		println!("Processor Architecture: {}", proc_info.u.s().wProcessorArchitecture);
	}
	println!("Page Size: {}", proc_info.dwPageSize);
	println!("Minimum Application Address: {:?}", proc_info.lpMinimumApplicationAddress);
	println!("Maximum Application Address: {:?}", proc_info.lpMaximumApplicationAddress);
	println!("Active Processor Mask: {:?}", proc_info.dwActiveProcessorMask);
	println!("Number of Processors: {}", proc_info.dwNumberOfProcessors);
	println!("Processor Type: {}", proc_info.dwProcessorType);
	println!("Allocation Granularity: {}", proc_info.dwAllocationGranularity);
	println!("Processor Level: {}", proc_info.wProcessorLevel);
	println!("Processor Revision: {}", proc_info.wProcessorRevision);
}

fn print_min_max_addr(min_app_addr: LPVOID, max_app_addr: LPVOID) {
	println!("min: {:p}, max: {:p}", min_app_addr, max_app_addr);
}

fn print_mem_info(mem_info: &MEMORY_BASIC_INFORMATION) {
	println!("Base Address: {:?}", mem_info.BaseAddress);
	println!("Allocation Base: {:?}", mem_info.AllocationBase);
	println!("Allocation Protect: {:?}", mem_info.AllocationProtect);
	println!("Region Size: {:?}", mem_info.RegionSize);
	println!("State: {:?}", mem_info.State);
	println!("Protect: {:?}", mem_info.Protect);
	println!("Type: {:?}", mem_info.Type);
}


// https://docs.rs/windows-sys/0.52.0/windows_sys/