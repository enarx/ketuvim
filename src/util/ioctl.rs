// Copyright 2019 Red Hat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]

use std::os::raw::c_ulong;

pub const KVM_GET_MSR_INDEX_LIST: c_ulong = 3221532162;

pub const KVM_GET_SUPPORTED_CPUID: c_ulong = 3221794309;
pub const KVM_GET_EMULATED_CPUID: c_ulong = 3221794313;
pub const KVM_GET_MSR_FEATURE_INDEX_LIST: c_ulong = 3221532170;
pub const KVM_SET_MEMORY_REGION: c_ulong = 1075359296;

pub const KVM_GET_DIRTY_LOG: c_ulong = 1074835010;
pub const KVM_SET_MEMORY_ALIAS: c_ulong = 1075883587;
pub const KVM_SET_NR_MMU_PAGES: c_ulong = 44612;
pub const KVM_GET_NR_MMU_PAGES: c_ulong = 44613;

pub const KVM_SET_TSS_ADDR: c_ulong = 44615;
pub const KVM_SET_IDENTITY_MAP_ADDR: c_ulong = 1074310728;
pub const KVM_CREATE_IRQCHIP: c_ulong = 44640;
pub const KVM_IRQ_LINE: c_ulong = 1074310753;
pub const KVM_GET_IRQCHIP: c_ulong = 3255348834;
pub const KVM_SET_IRQCHIP: c_ulong = 2181607011;
pub const KVM_CREATE_PIT: c_ulong = 44644;
pub const KVM_GET_PIT: c_ulong = 3225988709;
pub const KVM_SET_PIT: c_ulong = 2152246886;
pub const KVM_IRQ_LINE_STATUS: c_ulong = 3221794407;
pub const KVM_ASSIGN_PCI_DEVICE: c_ulong = 2151722601;
pub const KVM_SET_GSI_ROUTING: c_ulong = 1074310762;
pub const KVM_ASSIGN_DEV_IRQ: c_ulong = 1077980784;
pub const KVM_REINJECT_CONTROL: c_ulong = 44657;
pub const KVM_DEASSIGN_PCI_DEVICE: c_ulong = 1077980786;
pub const KVM_ASSIGN_SET_MSIX_NR: c_ulong = 1074310771;
pub const KVM_ASSIGN_SET_MSIX_ENTRY: c_ulong = 1074835060;
pub const KVM_DEASSIGN_DEV_IRQ: c_ulong = 1077980789;
pub const KVM_IRQFD: c_ulong = 1075883638;
pub const KVM_CREATE_PIT2: c_ulong = 1077980791;
pub const KVM_SET_BOOT_CPU_ID: c_ulong = 44664;
pub const KVM_IOEVENTFD: c_ulong = 1077980793;
pub const KVM_XEN_HVM_CONFIG: c_ulong = 1077456506;
pub const KVM_SET_CLOCK: c_ulong = 1076932219;
pub const KVM_GET_CLOCK: c_ulong = 2150674044;
pub const KVM_GET_PIT2: c_ulong = 2154868383;
pub const KVM_SET_PIT2: c_ulong = 1081126560;
pub const KVM_SET_TSC_KHZ: c_ulong = 44706;
pub const KVM_GET_TSC_KHZ: c_ulong = 44707;
pub const KVM_ASSIGN_SET_INTX_MASK: c_ulong = 1077980836;
pub const KVM_SIGNAL_MSI: c_ulong = 1075883685;
pub const KVM_ARM_SET_DEVICE_ADDR: c_ulong = 1074835115;
pub const KVM_CREATE_DEVICE: c_ulong = 3222056672;
pub const KVM_SET_DEVICE_ATTR: c_ulong = 1075359457;
pub const KVM_GET_DEVICE_ATTR: c_ulong = 1075359458;
pub const KVM_HAS_DEVICE_ATTR: c_ulong = 1075359459;


pub const KVM_TRANSLATE: c_ulong = 3222843013;
pub const KVM_INTERRUPT: c_ulong = 1074048646;
pub const KVM_GET_MSRS: c_ulong = 3221794440;
pub const KVM_SET_MSRS: c_ulong = 1074310793;
pub const KVM_SET_CPUID: c_ulong = 1074310794;
pub const KVM_SET_SIGNAL_MASK: c_ulong = 1074048651;
pub const KVM_GET_FPU: c_ulong = 2174791308;
pub const KVM_SET_FPU: c_ulong = 1101049485;
pub const KVM_GET_LAPIC: c_ulong = 2214637198;
pub const KVM_SET_LAPIC: c_ulong = 1140895375;
pub const KVM_SET_CPUID2: c_ulong = 1074310800;
pub const KVM_GET_CPUID2: c_ulong = 3221794449;
pub const KVM_TPR_ACCESS_REPORTING: c_ulong = 3223891602;
pub const KVM_SET_VAPIC_ADDR: c_ulong = 1074310803;
pub const KVM_GET_MP_STATE: c_ulong = 2147790488;
pub const KVM_SET_MP_STATE: c_ulong = 1074048665;
pub const KVM_NMI: c_ulong = 44698;
pub const KVM_SET_GUEST_DEBUG: c_ulong = 1078505115;
pub const KVM_X86_SETUP_MCE: c_ulong = 1074310812;
pub const KVM_X86_GET_MCE_CAP_SUPPORTED: c_ulong = 2148052637;
pub const KVM_X86_SET_MCE: c_ulong = 1077980830;
pub const KVM_GET_VCPU_EVENTS: c_ulong = 2151722655;
pub const KVM_SET_VCPU_EVENTS: c_ulong = 1077980832;
pub const KVM_GET_DEBUGREGS: c_ulong = 2155916961;
pub const KVM_SET_DEBUGREGS: c_ulong = 1082175138;
pub const KVM_ENABLE_CAP: c_ulong = 1080602275;
pub const KVM_GET_XSAVE: c_ulong = 2415963812;
pub const KVM_SET_XSAVE: c_ulong = 1342221989;
pub const KVM_GET_XCRS: c_ulong = 2173218470;
pub const KVM_SET_XCRS: c_ulong = 1099476647;
pub const KVM_DIRTY_TLB: c_ulong = 1074835114;
pub const KVM_GET_ONE_REG: c_ulong = 1074835115;
pub const KVM_SET_ONE_REG: c_ulong = 1074835116;
pub const KVM_KVMCLOCK_CTRL: c_ulong = 44717;
pub const KVM_GET_REG_LIST: c_ulong = 3221794480;
pub const KVM_SMI: c_ulong = 44727;
pub const KVM_MEMORY_ENCRYPT_REG_REGION: c_ulong = 2148576955;
pub const KVM_MEMORY_ENCRYPT_UNREG_REGION: c_ulong = 2148576956;
pub const KVM_HYPERV_EVENTFD: c_ulong = 1075359421;
pub const KVM_GET_NESTED_STATE: c_ulong = 3229658814;
pub const KVM_SET_NESTED_STATE: c_ulong = 1082175167;
