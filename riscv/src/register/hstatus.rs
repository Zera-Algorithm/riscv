read_csr_as_usize!(0x600);
write_csr_as_usize!(0x600);

// use super::sstatus;

pub enum VirtualizationMode {
    Host = 0,
    Guest = 1,
}

// pub enum PreviousMode {
//     U_mode,
//     HS_mode,
//     M_mode,
//     VU_mode,
//     VS_mode 
// }

pub fn set_spv(mode: VirtualizationMode) {
    let hstatus = read();
    let spv_mask = !(0b1 << 7 as usize);
    write((hstatus & spv_mask) | ((mode as usize) << 7))
}

pub fn read_spv() -> VirtualizationMode {
    let hstatus = read();
    if ((hstatus >> 7) & 0b1) == 0b0 {
        VirtualizationMode::Host
    } else {
        VirtualizationMode::Guest
    }
}

// pub fn previous_mode() -> Result<PreviousMode, Error> {
//     let spv = read_spv();
//     let spp = sstatus::read_spp();
//     match spv {
//         super::VirtualizationMode::Host => {
//             match spp {
//                 super::CpuMode::M => Err(core::fmt::Error),
//                 super::CpuMode::S => Ok(PreviousMode::HS_mode),
//                 super::CpuMode::U => Ok(PreviousMode::U_mode),
//             }
//         },
//         super::VirtualizationMode::Guest => {
//             match spp {
//                 super::CpuMode::M => Err(core::fmt::Error),
//                 super::CpuMode::S => Ok(PreviousMode::VS_mode),
//                 super::CpuMode::U => Ok(PreviousMode::VU_mode),
//             }
//         },
//     }
// }