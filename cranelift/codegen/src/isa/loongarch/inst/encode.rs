// DON'T EDIT, Generated automatically by https://github.com/xtexx/cranelift-loongarch
#![allow(unused)]

use super::*;
use crate::isa::loongarch::lower::isle::generated_code::*;
use crate::machinst::isle::WritableReg;

#[inline]
fn unsigned_field_width(value: u32, width: u8) -> u32 {
    debug_assert_eq!(value & (!0 << width), 0);
    value
}

#[inline]
fn signed_field_width(value: u32, width: u8) -> u32 {
    value & ((1 << width) - 1)
}

pub(crate) fn encode(inst: &LInst) -> u32 {
    match inst {
        LInst::FdmFjFk { op, fd, fj, fk } => {
            (match op {
                FdmFjFkOp::FADD_D => 0x01010000,
                FdmFjFkOp::FADD_S => 0x01008000,
                FdmFjFkOp::FCOPYSIGN_D => 0x01130000,
                FdmFjFkOp::FCOPYSIGN_S => 0x01128000,
                FdmFjFkOp::FDIV_D => 0x01070000,
                FdmFjFkOp::FDIV_S => 0x01068000,
                FdmFjFkOp::FMAX_D => 0x01090000,
                FdmFjFkOp::FMAX_S => 0x01088000,
                FdmFjFkOp::FMAXA_D => 0x010d0000,
                FdmFjFkOp::FMAXA_S => 0x010c8000,
                FdmFjFkOp::FMIN_D => 0x010b0000,
                FdmFjFkOp::FMIN_S => 0x010a8000,
                FdmFjFkOp::FMINA_D => 0x010f0000,
                FdmFjFkOp::FMINA_S => 0x010e8000,
                FdmFjFkOp::FMUL_D => 0x01050000,
                FdmFjFkOp::FMUL_S => 0x01048000,
                FdmFjFkOp::FSCALEB_D => 0x01110000,
                FdmFjFkOp::FSCALEB_S => 0x01108000,
                FdmFjFkOp::FSUB_D => 0x01030000,
                FdmFjFkOp::FSUB_S => 0x01028000,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
                | (reg_to_num(*fk) << 10)
        }
        LInst::FdmFjFkCa {
            op,
            fd,
            fj,
            fk,
            fcc_a,
        } => {
            (match op {
                FdmFjFkCaOp::FSEL => 0x0d000000,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
                | (reg_to_num(*fk) << 10)
                | (reg_to_num(*fcc_a) << 15)
        }
        LInst::DmJSk16ps2 { op, rd, rj, imm1 } => {
            (match op {
                DmJSk16ps2Op::JIRL => 0x4c000000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | ((signed_field_width(imm1.bits(), 16) >> 2) << 10)
        }
        LInst::FdJK { op, fd, rj, rk } => {
            (match op {
                FdJKOp::FSTX_D => 0x383c0000,
                FdJKOp::FSTX_S => 0x38380000,
            }) | (reg_to_num(*fd) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
        }
        LInst::Ud5JK { op, imm1, rj, rk } => {
            (match op {
                Ud5JKOp::PRELDX => 0x382c0000,
            }) | (unsigned_field_width(imm1.bits(), 5) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
        }
        LInst::DmJK { op, rd, rj, rk } => {
            (match op {
                DmJKOp::ADD_D => 0x00108000,
                DmJKOp::ADD_W => 0x00100000,
                DmJKOp::AND => 0x00148000,
                DmJKOp::ANDN => 0x00168000,
                DmJKOp::CRC_W_B_W => 0x00240000,
                DmJKOp::CRC_W_D_W => 0x00258000,
                DmJKOp::CRC_W_H_W => 0x00248000,
                DmJKOp::CRC_W_W_W => 0x00250000,
                DmJKOp::CRCC_W_B_W => 0x00260000,
                DmJKOp::CRCC_W_D_W => 0x00278000,
                DmJKOp::CRCC_W_H_W => 0x00268000,
                DmJKOp::CRCC_W_W_W => 0x00270000,
                DmJKOp::DIV_D => 0x00220000,
                DmJKOp::DIV_DU => 0x00230000,
                DmJKOp::DIV_W => 0x00200000,
                DmJKOp::DIV_WU => 0x00210000,
                DmJKOp::LDGT_B => 0x38780000,
                DmJKOp::LDGT_D => 0x38798000,
                DmJKOp::LDGT_H => 0x38788000,
                DmJKOp::LDGT_W => 0x38790000,
                DmJKOp::LDLE_B => 0x387a0000,
                DmJKOp::LDLE_D => 0x387b8000,
                DmJKOp::LDLE_H => 0x387a8000,
                DmJKOp::LDLE_W => 0x387b0000,
                DmJKOp::LDX_B => 0x38000000,
                DmJKOp::LDX_BU => 0x38200000,
                DmJKOp::LDX_D => 0x380c0000,
                DmJKOp::LDX_H => 0x38040000,
                DmJKOp::LDX_HU => 0x38240000,
                DmJKOp::LDX_W => 0x38080000,
                DmJKOp::LDX_WU => 0x38280000,
                DmJKOp::MASKEQZ => 0x00130000,
                DmJKOp::MASKNEZ => 0x00138000,
                DmJKOp::MOD_D => 0x00228000,
                DmJKOp::MOD_DU => 0x00238000,
                DmJKOp::MOD_W => 0x00208000,
                DmJKOp::MOD_WU => 0x00218000,
                DmJKOp::MUL_D => 0x001d8000,
                DmJKOp::MUL_W => 0x001c0000,
                DmJKOp::MULH_D => 0x001e0000,
                DmJKOp::MULH_DU => 0x001e8000,
                DmJKOp::MULH_W => 0x001c8000,
                DmJKOp::MULH_WU => 0x001d0000,
                DmJKOp::MULW_D_W => 0x001f0000,
                DmJKOp::MULW_D_WU => 0x001f8000,
                DmJKOp::NOR => 0x00140000,
                DmJKOp::OR => 0x00150000,
                DmJKOp::ORN => 0x00160000,
                DmJKOp::ROTR_D => 0x001b8000,
                DmJKOp::ROTR_W => 0x001b0000,
                DmJKOp::SLL_D => 0x00188000,
                DmJKOp::SLL_W => 0x00170000,
                DmJKOp::SLT => 0x00120000,
                DmJKOp::SLTU => 0x00128000,
                DmJKOp::SRA_D => 0x00198000,
                DmJKOp::SRA_W => 0x00180000,
                DmJKOp::SRL_D => 0x00190000,
                DmJKOp::SRL_W => 0x00178000,
                DmJKOp::SUB_D => 0x00118000,
                DmJKOp::SUB_W => 0x00110000,
                DmJKOp::XOR => 0x00158000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
        }
        LInst::JSd5Sk16ps2 { op, rj, imm1, imm2 } => {
            (match op {
                JSd5Sk16ps2Op::BEQZ => 0x40000000,
                JSd5Sk16ps2Op::BNEZ => 0x44000000,
            }) | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm1.bits(), 5) << 0)
                | ((signed_field_width(imm2.bits(), 16) >> 2) << 10)
        }
        LInst::FdJSk12 { op, fd, rj, imm1 } => {
            (match op {
                FdJSk12Op::FST_D => 0x2bc00000,
                FdJSk12Op::FST_S => 0x2b400000,
            }) | (reg_to_num(*fd) << 0)
                | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm1.bits(), 12) << 10)
        }
        LInst::DmSj20 { op, rd, imm1 } => {
            (match op {
                DmSj20Op::LU12I_W => 0x14000000,
                DmSj20Op::LU32I_D => 0x16000000,
                DmSj20Op::PCADDI => 0x18000000,
                DmSj20Op::PCADDU12I => 0x1c000000,
                DmSj20Op::PCADDU18I => 0x1e000000,
                DmSj20Op::PCALAU12I => 0x1a000000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (signed_field_width(imm1.bits(), 20) << 5)
        }
        LInst::FdmJK { op, fd, rj, rk } => {
            (match op {
                FdmJKOp::FLDX_D => 0x38340000,
                FdmJKOp::FLDX_S => 0x38300000,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
        }
        LInst::DJSk14ps2 { op, rd, rj, imm1 } => {
            (match op {
                DJSk14ps2Op::STPTR_D => 0x27000000,
                DJSk14ps2Op::STPTR_W => 0x25000000,
            }) | (reg_to_num(*rd) << 0)
                | (reg_to_num(*rj) << 5)
                | ((signed_field_width(imm1.bits(), 14) >> 2) << 10)
        }
        LInst::DmJKUa2pp1 {
            op,
            rd,
            rj,
            rk,
            imm1,
        } => {
            (match op {
                DmJKUa2pp1Op::ALSL_D => 0x002c0000,
                DmJKUa2pp1Op::ALSL_W => 0x00040000,
                DmJKUa2pp1Op::ALSL_WU => 0x00060000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
                | ((unsigned_field_width(imm1.bits(), 2) - 1) << 15)
        }
        LInst::DmJ { op, rd, rj } => {
            (match op {
                DmJOp::BITREV_4B => 0x00004800,
                DmJOp::BITREV_8B => 0x00004c00,
                DmJOp::BITREV_D => 0x00005400,
                DmJOp::BITREV_W => 0x00005000,
                DmJOp::CLO_D => 0x00002000,
                DmJOp::CLO_W => 0x00001000,
                DmJOp::CLZ_D => 0x00002400,
                DmJOp::CLZ_W => 0x00001400,
                DmJOp::CPUCFG => 0x00006c00,
                DmJOp::CTO_D => 0x00002800,
                DmJOp::CTO_W => 0x00001800,
                DmJOp::CTZ_D => 0x00002c00,
                DmJOp::CTZ_W => 0x00001c00,
                DmJOp::EXT_W_B => 0x00005c00,
                DmJOp::EXT_W_H => 0x00005800,
                DmJOp::LLACQ_D => 0x38578800,
                DmJOp::LLACQ_W => 0x38578000,
                DmJOp::MOVFCSR2GR => 0x0114c800,
                DmJOp::REVB_2H => 0x00003000,
                DmJOp::REVB_2W => 0x00003800,
                DmJOp::REVB_4H => 0x00003400,
                DmJOp::REVB_D => 0x00003c00,
                DmJOp::REVH_2W => 0x00004000,
                DmJOp::REVH_D => 0x00004400,
                DmJOp::SCREL_D => 0x38578c00,
                DmJOp::SCREL_W => 0x38578400,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
        }
        LInst::FdmCj { op, fd, fcc_j } => {
            (match op {
                FdmCjOp::MOVCF2FR => 0x0114d400,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*fcc_j) << 5)
        }
        LInst::CjSd5Sk16ps2 {
            op,
            fcc_j,
            imm1,
            imm2,
        } => {
            (match op {
                CjSd5Sk16ps2Op::BCEQZ => 0x48000000,
                CjSd5Sk16ps2Op::BCNEZ => 0x48000100,
            }) | (reg_to_num(*fcc_j) << 5)
                | (signed_field_width(imm1.bits(), 5) << 0)
                | ((signed_field_width(imm2.bits(), 16) >> 2) << 10)
        }
        LInst::DmJUm5Uk5 {
            op,
            rd,
            rj,
            imm1,
            imm2,
        } => {
            (match op {
                DmJUm5Uk5Op::BSTRINS_W => 0x00600000,
                DmJUm5Uk5Op::BSTRPICK_W => 0x00608000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (unsigned_field_width(imm1.bits(), 5) << 16)
                | (unsigned_field_width(imm2.bits(), 5) << 10)
        }
        LInst::DmJKUa2 {
            op,
            rd,
            rj,
            rk,
            imm1,
        } => {
            (match op {
                DmJKUa2Op::BYTEPICK_W => 0x00080000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
                | (unsigned_field_width(imm1.bits(), 2) << 15)
        }
        LInst::DmKmJ { op, rd, rk, rj } => {
            (match op {
                DmKmJOp::AMADD_B => 0x385d0000,
                DmKmJOp::AMADD_D => 0x38618000,
                DmKmJOp::AMADD_H => 0x385d8000,
                DmKmJOp::AMADD_W => 0x38610000,
                DmKmJOp::AMADD_DB_B => 0x385f0000,
                DmKmJOp::AMADD_DB_H => 0x385f8000,
                DmKmJOp::AMADD_DB_W => 0x386a0000,
                DmKmJOp::AMSWAP_B => 0x385c0000,
                DmKmJOp::AMSWAP_D => 0x38608000,
                DmKmJOp::AMSWAP_H => 0x385c8000,
                DmKmJOp::AMSWAP_W => 0x38600000,
                DmKmJOp::AMSWAP_DB_B => 0x385e0000,
                DmKmJOp::AMSWAP_DB_H => 0x385e8000,
                DmKmJOp::AMSWAP_DB_W => 0x38690000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(rk.to_reg()) << 10)
                | (reg_to_num(*rj) << 5)
        }
        LInst::DmFj { op, rd, fj } => {
            (match op {
                DmFjOp::MOVFR2GR_D => 0x0114b800,
                DmFjOp::MOVFR2GR_S => 0x0114b400,
                DmFjOp::MOVFRH2GR_S => 0x0114bc00,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
        }
        LInst::FdmFj { op, fd, fj } => {
            (match op {
                FdmFjOp::FABS_D => 0x01140800,
                FdmFjOp::FABS_S => 0x01140400,
                FdmFjOp::FCLASS_D => 0x01143800,
                FdmFjOp::FCLASS_S => 0x01143400,
                FdmFjOp::FCVT_D_S => 0x01192400,
                FdmFjOp::FCVT_S_D => 0x01191800,
                FdmFjOp::FFINT_D_L => 0x011d2800,
                FdmFjOp::FFINT_D_W => 0x011d2000,
                FdmFjOp::FFINT_S_L => 0x011d1800,
                FdmFjOp::FFINT_S_W => 0x011d1000,
                FdmFjOp::FLOGB_D => 0x01142800,
                FdmFjOp::FLOGB_S => 0x01142400,
                FdmFjOp::FMOV_D => 0x01149800,
                FdmFjOp::FMOV_S => 0x01149400,
                FdmFjOp::FNEG_D => 0x01141800,
                FdmFjOp::FNEG_S => 0x01141400,
                FdmFjOp::FRECIP_D => 0x01145800,
                FdmFjOp::FRECIP_S => 0x01145400,
                FdmFjOp::FRECIPE_D => 0x01147800,
                FdmFjOp::FRECIPE_S => 0x01147400,
                FdmFjOp::FRINT_D => 0x011e4800,
                FdmFjOp::FRINT_S => 0x011e4400,
                FdmFjOp::FRSQRT_D => 0x01146800,
                FdmFjOp::FRSQRT_S => 0x01146400,
                FdmFjOp::FRSQRTE_D => 0x01148800,
                FdmFjOp::FRSQRTE_S => 0x01148400,
                FdmFjOp::FSQRT_D => 0x01144800,
                FdmFjOp::FSQRT_S => 0x01144400,
                FdmFjOp::FTINT_L_D => 0x011b2800,
                FdmFjOp::FTINT_L_S => 0x011b2400,
                FdmFjOp::FTINT_W_D => 0x011b0800,
                FdmFjOp::FTINT_W_S => 0x011b0400,
                FdmFjOp::FTINTRM_L_D => 0x011a2800,
                FdmFjOp::FTINTRM_L_S => 0x011a2400,
                FdmFjOp::FTINTRM_W_D => 0x011a0800,
                FdmFjOp::FTINTRM_W_S => 0x011a0400,
                FdmFjOp::FTINTRNE_L_D => 0x011ae800,
                FdmFjOp::FTINTRNE_L_S => 0x011ae400,
                FdmFjOp::FTINTRNE_W_D => 0x011ac800,
                FdmFjOp::FTINTRNE_W_S => 0x011ac400,
                FdmFjOp::FTINTRP_L_D => 0x011a6800,
                FdmFjOp::FTINTRP_L_S => 0x011a6400,
                FdmFjOp::FTINTRP_W_D => 0x011a4800,
                FdmFjOp::FTINTRP_W_S => 0x011a4400,
                FdmFjOp::FTINTRZ_L_D => 0x011aa800,
                FdmFjOp::FTINTRZ_L_S => 0x011aa400,
                FdmFjOp::FTINTRZ_W_D => 0x011a8800,
                FdmFjOp::FTINTRZ_W_S => 0x011a8400,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
        }
        LInst::DmJKUa3 {
            op,
            rd,
            rj,
            rk,
            imm1,
        } => {
            (match op {
                DmJKUa3Op::BYTEPICK_D => 0x000c0000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
                | (unsigned_field_width(imm1.bits(), 3) << 15)
        }
        LInst::DmJUk5 { op, rd, rj, imm1 } => {
            (match op {
                DmJUk5Op::ROTRI_W => 0x004c8000,
                DmJUk5Op::SLLI_W => 0x00408000,
                DmJUk5Op::SRAI_W => 0x00488000,
                DmJUk5Op::SRLI_W => 0x00448000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (unsigned_field_width(imm1.bits(), 5) << 10)
        }
        LInst::JK { op, rj, rk } => {
            (match op {
                JKOp::ASRTGT_D => 0x00018000,
                JKOp::ASRTLE_D => 0x00010000,
            }) | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
        }
        LInst::DmJSk12 { op, rd, rj, imm1 } => {
            (match op {
                DmJSk12Op::ADDI_D => 0x02c00000,
                DmJSk12Op::ADDI_W => 0x02800000,
                DmJSk12Op::LD_B => 0x28000000,
                DmJSk12Op::LD_BU => 0x2a000000,
                DmJSk12Op::LD_D => 0x28c00000,
                DmJSk12Op::LD_H => 0x28400000,
                DmJSk12Op::LD_HU => 0x2a400000,
                DmJSk12Op::LD_W => 0x28800000,
                DmJSk12Op::LD_WU => 0x2a800000,
                DmJSk12Op::LU52I_D => 0x03000000,
                DmJSk12Op::SLTI => 0x02000000,
                DmJSk12Op::SLTUI => 0x02400000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm1.bits(), 12) << 10)
        }
        LInst::DmJSk14ps2 { op, rd, rj, imm1 } => {
            (match op {
                DmJSk14ps2Op::LDPTR_D => 0x26000000,
                DmJSk14ps2Op::LDPTR_W => 0x24000000,
                DmJSk14ps2Op::LL_D => 0x22000000,
                DmJSk14ps2Op::LL_W => 0x20000000,
                DmJSk14ps2Op::SC_D => 0x23000000,
                DmJSk14ps2Op::SC_W => 0x21000000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | ((signed_field_width(imm1.bits(), 14) >> 2) << 10)
        }
        LInst::DJm { op, rd, rj } => {
            (match op {
                DJmOp::MOVGR2FCSR => 0x0114c000,
            }) | (reg_to_num(*rd) << 0)
                | (reg_to_num(rj.to_reg()) << 5)
        }
        LInst::CdmFj { op, fcc_d, fj } => {
            (match op {
                CdmFjOp::MOVFR2CF => 0x0114d000,
            }) | (reg_to_num(fcc_d.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
        }
        LInst::JDSk16ps2 { op, rj, rd, imm1 } => {
            (match op {
                JDSk16ps2Op::BEQ => 0x58000000,
                JDSk16ps2Op::BGE => 0x64000000,
                JDSk16ps2Op::BGEU => 0x6c000000,
                JDSk16ps2Op::BLT => 0x60000000,
                JDSk16ps2Op::BLTU => 0x68000000,
                JDSk16ps2Op::BNE => 0x5c000000,
            }) | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rd) << 0)
                | ((signed_field_width(imm1.bits(), 16) >> 2) << 10)
        }
        LInst::DmJUk12 { op, rd, rj, imm1 } => {
            (match op {
                DmJUk12Op::ANDI => 0x03400000,
                DmJUk12Op::ORI => 0x03800000,
                DmJUk12Op::XORI => 0x03c00000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (unsigned_field_width(imm1.bits(), 12) << 10)
        }
        LInst::Ud15 { op, imm1 } => {
            (match op {
                Ud15Op::BREAK => 0x002a0000,
                Ud15Op::DBAR => 0x38720000,
                Ud15Op::DBCL => 0x002a8000,
                Ud15Op::IBAR => 0x38728000,
                Ud15Op::SYSCALL => 0x002b0000,
            }) | (unsigned_field_width(imm1.bits(), 15) << 0)
        }
        LInst::CdmFjFk { op, fcc_d, fj, fk } => {
            (match op {
                CdmFjFkOp::FCMP_CAF_D => 0x0c200000,
                CdmFjFkOp::FCMP_CAF_S => 0x0c100000,
                CdmFjFkOp::FCMP_CEQ_D => 0x0c220000,
                CdmFjFkOp::FCMP_CEQ_S => 0x0c120000,
                CdmFjFkOp::FCMP_CLE_D => 0x0c230000,
                CdmFjFkOp::FCMP_CLE_S => 0x0c130000,
                CdmFjFkOp::FCMP_CLT_D => 0x0c210000,
                CdmFjFkOp::FCMP_CLT_S => 0x0c110000,
                CdmFjFkOp::FCMP_CNE_D => 0x0c280000,
                CdmFjFkOp::FCMP_CNE_S => 0x0c180000,
                CdmFjFkOp::FCMP_COR_D => 0x0c2a0000,
                CdmFjFkOp::FCMP_COR_S => 0x0c1a0000,
                CdmFjFkOp::FCMP_CUEQ_D => 0x0c260000,
                CdmFjFkOp::FCMP_CUEQ_S => 0x0c160000,
                CdmFjFkOp::FCMP_CULE_D => 0x0c270000,
                CdmFjFkOp::FCMP_CULE_S => 0x0c170000,
                CdmFjFkOp::FCMP_CULT_D => 0x0c250000,
                CdmFjFkOp::FCMP_CULT_S => 0x0c150000,
                CdmFjFkOp::FCMP_CUN_D => 0x0c240000,
                CdmFjFkOp::FCMP_CUN_S => 0x0c140000,
                CdmFjFkOp::FCMP_CUNE_D => 0x0c2c0000,
                CdmFjFkOp::FCMP_CUNE_S => 0x0c1c0000,
                CdmFjFkOp::FCMP_SAF_D => 0x0c208000,
                CdmFjFkOp::FCMP_SAF_S => 0x0c108000,
                CdmFjFkOp::FCMP_SEQ_D => 0x0c228000,
                CdmFjFkOp::FCMP_SEQ_S => 0x0c128000,
                CdmFjFkOp::FCMP_SLE_D => 0x0c238000,
                CdmFjFkOp::FCMP_SLE_S => 0x0c138000,
                CdmFjFkOp::FCMP_SLT_D => 0x0c218000,
                CdmFjFkOp::FCMP_SLT_S => 0x0c118000,
                CdmFjFkOp::FCMP_SNE_D => 0x0c288000,
                CdmFjFkOp::FCMP_SNE_S => 0x0c188000,
                CdmFjFkOp::FCMP_SOR_D => 0x0c2a8000,
                CdmFjFkOp::FCMP_SOR_S => 0x0c1a8000,
                CdmFjFkOp::FCMP_SUEQ_D => 0x0c268000,
                CdmFjFkOp::FCMP_SUEQ_S => 0x0c168000,
                CdmFjFkOp::FCMP_SULE_D => 0x0c278000,
                CdmFjFkOp::FCMP_SULE_S => 0x0c178000,
                CdmFjFkOp::FCMP_SULT_D => 0x0c258000,
                CdmFjFkOp::FCMP_SULT_S => 0x0c158000,
                CdmFjFkOp::FCMP_SUN_D => 0x0c248000,
                CdmFjFkOp::FCMP_SUN_S => 0x0c148000,
                CdmFjFkOp::FCMP_SUNE_D => 0x0c2c8000,
                CdmFjFkOp::FCMP_SUNE_S => 0x0c1c8000,
            }) | (reg_to_num(fcc_d.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
                | (reg_to_num(*fk) << 10)
        }
        LInst::DJSk12 { op, rd, rj, imm1 } => {
            (match op {
                DJSk12Op::ST_B => 0x29000000,
                DJSk12Op::ST_D => 0x29c00000,
                DJSk12Op::ST_H => 0x29400000,
                DJSk12Op::ST_W => 0x29800000,
            }) | (reg_to_num(*rd) << 0)
                | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm1.bits(), 12) << 10)
        }
        LInst::DJK { op, rd, rj, rk } => {
            (match op {
                DJKOp::STGT_B => 0x387c0000,
                DJKOp::STGT_D => 0x387d8000,
                DJKOp::STGT_H => 0x387c8000,
                DJKOp::STGT_W => 0x387d0000,
                DJKOp::STLE_B => 0x387e0000,
                DJKOp::STLE_D => 0x387f8000,
                DJKOp::STLE_H => 0x387e8000,
                DJKOp::STLE_W => 0x387f0000,
                DJKOp::STX_B => 0x38100000,
                DJKOp::STX_D => 0x381c0000,
                DJKOp::STX_H => 0x38140000,
                DJKOp::STX_W => 0x38180000,
            }) | (reg_to_num(*rd) << 0)
                | (reg_to_num(*rj) << 5)
                | (reg_to_num(*rk) << 10)
        }
        LInst::FdmFjFkFa { op, fd, fj, fk, fa } => {
            (match op {
                FdmFjFkFaOp::FMADD_D => 0x08200000,
                FdmFjFkFaOp::FMADD_S => 0x08100000,
                FdmFjFkFaOp::FMSUB_D => 0x08600000,
                FdmFjFkFaOp::FMSUB_S => 0x08500000,
                FdmFjFkFaOp::FNMADD_D => 0x08a00000,
                FdmFjFkFaOp::FNMADD_S => 0x08900000,
                FdmFjFkFaOp::FNMSUB_D => 0x08e00000,
                FdmFjFkFaOp::FNMSUB_S => 0x08d00000,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*fj) << 5)
                | (reg_to_num(*fk) << 10)
                | (reg_to_num(*fa) << 15)
        }
        LInst::CdmJ { op, fcc_d, rj } => {
            (match op {
                CdmJOp::MOVGR2CF => 0x0114d800,
            }) | (reg_to_num(fcc_d.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
        }
        LInst::DmJSk16 { op, rd, rj, imm1 } => {
            (match op {
                DmJSk16Op::ADDU16I_D => 0x10000000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm1.bits(), 16) << 10)
        }
        LInst::DmKJ { op, rd, rk, rj } => {
            (match op {
                DmKJOp::AMADD_DB_D => 0x386a8000,
                DmKJOp::AMAND_D => 0x38628000,
                DmKJOp::AMAND_W => 0x38620000,
                DmKJOp::AMAND_DB_D => 0x386b8000,
                DmKJOp::AMAND_DB_W => 0x386b0000,
                DmKJOp::AMCAS_B => 0x38580000,
                DmKJOp::AMCAS_D => 0x38598000,
                DmKJOp::AMCAS_H => 0x38588000,
                DmKJOp::AMCAS_W => 0x38590000,
                DmKJOp::AMCAS_DB_B => 0x385a0000,
                DmKJOp::AMCAS_DB_D => 0x385b8000,
                DmKJOp::AMCAS_DB_H => 0x385a8000,
                DmKJOp::AMCAS_DB_W => 0x385b0000,
                DmKJOp::AMMAX_D => 0x38658000,
                DmKJOp::AMMAX_DU => 0x38678000,
                DmKJOp::AMMAX_W => 0x38650000,
                DmKJOp::AMMAX_WU => 0x38670000,
                DmKJOp::AMMAX_DB_D => 0x386e8000,
                DmKJOp::AMMAX_DB_DU => 0x38708000,
                DmKJOp::AMMAX_DB_W => 0x386e0000,
                DmKJOp::AMMAX_DB_WU => 0x38700000,
                DmKJOp::AMMIN_D => 0x38668000,
                DmKJOp::AMMIN_DU => 0x38688000,
                DmKJOp::AMMIN_W => 0x38660000,
                DmKJOp::AMMIN_WU => 0x38680000,
                DmKJOp::AMMIN_DB_D => 0x386f8000,
                DmKJOp::AMMIN_DB_DU => 0x38718000,
                DmKJOp::AMMIN_DB_W => 0x386f0000,
                DmKJOp::AMMIN_DB_WU => 0x38710000,
                DmKJOp::AMOR_D => 0x38638000,
                DmKJOp::AMOR_W => 0x38630000,
                DmKJOp::AMOR_DB_D => 0x386c8000,
                DmKJOp::AMOR_DB_W => 0x386c0000,
                DmKJOp::AMSWAP_DB_D => 0x38698000,
                DmKJOp::AMXOR_D => 0x38648000,
                DmKJOp::AMXOR_W => 0x38640000,
                DmKJOp::AMXOR_DB_D => 0x386d8000,
                DmKJOp::AMXOR_DB_W => 0x386d0000,
                DmKJOp::SC_Q => 0x38570000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rk) << 10)
                | (reg_to_num(*rj) << 5)
        }
        LInst::DmJUm6Uk6 {
            op,
            rd,
            rj,
            imm1,
            imm2,
        } => {
            (match op {
                DmJUm6Uk6Op::BSTRINS_D => 0x00800000,
                DmJUm6Uk6Op::BSTRPICK_D => 0x00c00000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (unsigned_field_width(imm1.bits(), 6) << 16)
                | (unsigned_field_width(imm2.bits(), 6) << 10)
        }
        LInst::Sd10Sk16ps2 { op, imm1, imm2 } => {
            (match op {
                Sd10Sk16ps2Op::B => 0x50000000,
                Sd10Sk16ps2Op::BL => 0x54000000,
            }) | (signed_field_width(imm1.bits(), 10) << 0)
                | ((signed_field_width(imm2.bits(), 16) >> 2) << 10)
        }
        LInst::FdmJ { op, fd, rj } => {
            (match op {
                FdmJOp::MOVGR2FR_D => 0x0114a800,
                FdmJOp::MOVGR2FR_W => 0x0114a400,
                FdmJOp::MOVGR2FRH_W => 0x0114ac00,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
        }
        LInst::DmJm { op, rd, rj } => {
            (match op {
                DmJmOp::RDTIME_D => 0x00006800,
                DmJmOp::RDTIMEH_W => 0x00006400,
                DmJmOp::RDTIMEL_W => 0x00006000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(rj.to_reg()) << 5)
        }
        LInst::FdmJSk12 { op, fd, rj, imm1 } => {
            (match op {
                FdmJSk12Op::FLD_D => 0x2b800000,
                FdmJSk12Op::FLD_S => 0x2b000000,
            }) | (reg_to_num(fd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm1.bits(), 12) << 10)
        }
        LInst::Ud5JSk12 { op, imm1, rj, imm2 } => {
            (match op {
                Ud5JSk12Op::PRELD => 0x2ac00000,
            }) | (unsigned_field_width(imm1.bits(), 5) << 0)
                | (reg_to_num(*rj) << 5)
                | (signed_field_width(imm2.bits(), 12) << 10)
        }
        LInst::DmJUk6 { op, rd, rj, imm1 } => {
            (match op {
                DmJUk6Op::ROTRI_D => 0x004d0000,
                DmJUk6Op::SLLI_D => 0x00410000,
                DmJUk6Op::SRAI_D => 0x00490000,
                DmJUk6Op::SRLI_D => 0x00450000,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*rj) << 5)
                | (unsigned_field_width(imm1.bits(), 6) << 10)
        }
        LInst::DmCj { op, rd, fcc_j } => {
            (match op {
                DmCjOp::MOVCF2GR => 0x0114dc00,
            }) | (reg_to_num(rd.to_reg()) << 0)
                | (reg_to_num(*fcc_j) << 5)
        }
    }
}

pub(crate) fn get_linst_operands(inst: &mut LInst, collector: &mut impl OperandVisitor) {
    match inst {
        LInst::FdmFjFk { fd, fj, fk, .. } => {
            collector.reg_use(fj);
            collector.reg_use(fk);
            collector.reg_def(fd);
        }
        LInst::FdmFjFkCa {
            fd, fj, fk, fcc_a, ..
        } => {
            collector.reg_use(fj);
            collector.reg_use(fk);
            collector.reg_use(fcc_a);
            collector.reg_def(fd);
        }
        LInst::DmJSk16ps2 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::FdJK { fd, rj, rk, .. } => {
            collector.reg_use(fd);
            collector.reg_use(rj);
            collector.reg_use(rk);
        }
        LInst::Ud5JK { rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
        }
        LInst::DmJK { rd, rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
            collector.reg_def(rd);
        }
        LInst::JSd5Sk16ps2 { rj, .. } => {
            collector.reg_use(rj);
        }
        LInst::FdJSk12 { fd, rj, .. } => {
            collector.reg_use(fd);
            collector.reg_use(rj);
        }
        LInst::DmSj20 { rd, .. } => {
            collector.reg_def(rd);
        }
        LInst::FdmJK { fd, rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
            collector.reg_def(fd);
        }
        LInst::DJSk14ps2 { rd, rj, .. } => {
            collector.reg_use(rd);
            collector.reg_use(rj);
        }
        LInst::DmJKUa2pp1 { rd, rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
            collector.reg_def(rd);
        }
        LInst::DmJ { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::FdmCj { fd, fcc_j, .. } => {
            collector.reg_use(fcc_j);
            collector.reg_def(fd);
        }
        LInst::CjSd5Sk16ps2 { fcc_j, .. } => {
            collector.reg_use(fcc_j);
        }
        LInst::DmJUm5Uk5 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::DmJKUa2 { rd, rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
            collector.reg_def(rd);
        }
        LInst::DmKmJ { rd, rk, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
            collector.reg_def(rk);
        }
        LInst::DmFj { rd, fj, .. } => {
            collector.reg_use(fj);
            collector.reg_def(rd);
        }
        LInst::FdmFj { fd, fj, .. } => {
            collector.reg_use(fj);
            collector.reg_def(fd);
        }
        LInst::DmJKUa3 { rd, rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
            collector.reg_def(rd);
        }
        LInst::DmJUk5 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::JK { rj, rk, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rk);
        }
        LInst::DmJSk12 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::DmJSk14ps2 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::DJm { rd, rj, .. } => {
            collector.reg_use(rd);
            collector.reg_def(rj);
        }
        LInst::CdmFj { fcc_d, fj, .. } => {
            collector.reg_use(fj);
            collector.reg_def(fcc_d);
        }
        LInst::JDSk16ps2 { rj, rd, .. } => {
            collector.reg_use(rj);
            collector.reg_use(rd);
        }
        LInst::DmJUk12 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::Ud15 { .. } => {}
        LInst::CdmFjFk { fcc_d, fj, fk, .. } => {
            collector.reg_use(fj);
            collector.reg_use(fk);
            collector.reg_def(fcc_d);
        }
        LInst::DJSk12 { rd, rj, .. } => {
            collector.reg_use(rd);
            collector.reg_use(rj);
        }
        LInst::DJK { rd, rj, rk, .. } => {
            collector.reg_use(rd);
            collector.reg_use(rj);
            collector.reg_use(rk);
        }
        LInst::FdmFjFkFa { fd, fj, fk, fa, .. } => {
            collector.reg_use(fj);
            collector.reg_use(fk);
            collector.reg_use(fa);
            collector.reg_def(fd);
        }
        LInst::CdmJ { fcc_d, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(fcc_d);
        }
        LInst::DmJSk16 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::DmKJ { rd, rk, rj, .. } => {
            collector.reg_use(rk);
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::DmJUm6Uk6 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::Sd10Sk16ps2 { .. } => {}
        LInst::FdmJ { fd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(fd);
        }
        LInst::DmJm { rd, rj, .. } => {
            collector.reg_def(rd);
            collector.reg_def(rj);
        }
        LInst::FdmJSk12 { fd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(fd);
        }
        LInst::Ud5JSk12 { rj, .. } => {
            collector.reg_use(rj);
        }
        LInst::DmJUk6 { rd, rj, .. } => {
            collector.reg_use(rj);
            collector.reg_def(rd);
        }
        LInst::DmCj { rd, fcc_j, .. } => {
            collector.reg_use(fcc_j);
            collector.reg_def(rd);
        }
    }
}

pub(crate) fn format_linst<'a>(inst: &LInst, f: &'a mut (dyn Write + 'a)) -> core::fmt::Result {
    match inst {
        LInst::FdmFjFk { op, fd, fj, fk } => {
            f.write_str(match op {
                FdmFjFkOp::FADD_D => "fadd.d",
                FdmFjFkOp::FADD_S => "fadd.s",
                FdmFjFkOp::FCOPYSIGN_D => "fcopysign.d",
                FdmFjFkOp::FCOPYSIGN_S => "fcopysign.s",
                FdmFjFkOp::FDIV_D => "fdiv.d",
                FdmFjFkOp::FDIV_S => "fdiv.s",
                FdmFjFkOp::FMAX_D => "fmax.d",
                FdmFjFkOp::FMAX_S => "fmax.s",
                FdmFjFkOp::FMAXA_D => "fmaxa.d",
                FdmFjFkOp::FMAXA_S => "fmaxa.s",
                FdmFjFkOp::FMIN_D => "fmin.d",
                FdmFjFkOp::FMIN_S => "fmin.s",
                FdmFjFkOp::FMINA_D => "fmina.d",
                FdmFjFkOp::FMINA_S => "fmina.s",
                FdmFjFkOp::FMUL_D => "fmul.d",
                FdmFjFkOp::FMUL_S => "fmul.s",
                FdmFjFkOp::FSCALEB_D => "fscaleb.d",
                FdmFjFkOp::FSCALEB_S => "fscaleb.s",
                FdmFjFkOp::FSUB_D => "fsub.d",
                FdmFjFkOp::FSUB_S => "fsub.s",
            })?;
            write!(
                f,
                " $f{}, $f{}, $f{}",
                reg_to_num(fd.to_reg()),
                reg_to_num(*fj),
                reg_to_num(*fk)
            )?;
        }
        LInst::FdmFjFkCa {
            op,
            fd,
            fj,
            fk,
            fcc_a,
        } => {
            f.write_str(match op {
                FdmFjFkCaOp::FSEL => "fsel",
            })?;
            write!(
                f,
                " $f{}, $f{}, $f{}, $fcc{}",
                reg_to_num(fd.to_reg()),
                reg_to_num(*fj),
                reg_to_num(*fk),
                reg_to_num(*fcc_a)
            )?;
        }
        LInst::DmJSk16ps2 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJSk16ps2Op::JIRL => "jirl",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::FdJK { op, fd, rj, rk } => {
            f.write_str(match op {
                FdJKOp::FSTX_D => "fstx.d",
                FdJKOp::FSTX_S => "fstx.s",
            })?;
            write!(
                f,
                " $f{}, $r{}, $r{}",
                reg_to_num(*fd),
                reg_to_num(*rj),
                reg_to_num(*rk)
            )?;
        }
        LInst::Ud5JK { op, imm1, rj, rk } => {
            f.write_str(match op {
                Ud5JKOp::PRELDX => "preldx",
            })?;
            write!(
                f,
                " {}, $r{}, $r{}",
                imm1.bits(),
                reg_to_num(*rj),
                reg_to_num(*rk)
            )?;
        }
        LInst::DmJK { op, rd, rj, rk } => {
            f.write_str(match op {
                DmJKOp::ADD_D => "add.d",
                DmJKOp::ADD_W => "add.w",
                DmJKOp::AND => "and",
                DmJKOp::ANDN => "andn",
                DmJKOp::CRC_W_B_W => "crc.w.b.w",
                DmJKOp::CRC_W_D_W => "crc.w.d.w",
                DmJKOp::CRC_W_H_W => "crc.w.h.w",
                DmJKOp::CRC_W_W_W => "crc.w.w.w",
                DmJKOp::CRCC_W_B_W => "crcc.w.b.w",
                DmJKOp::CRCC_W_D_W => "crcc.w.d.w",
                DmJKOp::CRCC_W_H_W => "crcc.w.h.w",
                DmJKOp::CRCC_W_W_W => "crcc.w.w.w",
                DmJKOp::DIV_D => "div.d",
                DmJKOp::DIV_DU => "div.du",
                DmJKOp::DIV_W => "div.w",
                DmJKOp::DIV_WU => "div.wu",
                DmJKOp::LDGT_B => "ldgt.b",
                DmJKOp::LDGT_D => "ldgt.d",
                DmJKOp::LDGT_H => "ldgt.h",
                DmJKOp::LDGT_W => "ldgt.w",
                DmJKOp::LDLE_B => "ldle.b",
                DmJKOp::LDLE_D => "ldle.d",
                DmJKOp::LDLE_H => "ldle.h",
                DmJKOp::LDLE_W => "ldle.w",
                DmJKOp::LDX_B => "ldx.b",
                DmJKOp::LDX_BU => "ldx.bu",
                DmJKOp::LDX_D => "ldx.d",
                DmJKOp::LDX_H => "ldx.h",
                DmJKOp::LDX_HU => "ldx.hu",
                DmJKOp::LDX_W => "ldx.w",
                DmJKOp::LDX_WU => "ldx.wu",
                DmJKOp::MASKEQZ => "maskeqz",
                DmJKOp::MASKNEZ => "masknez",
                DmJKOp::MOD_D => "mod.d",
                DmJKOp::MOD_DU => "mod.du",
                DmJKOp::MOD_W => "mod.w",
                DmJKOp::MOD_WU => "mod.wu",
                DmJKOp::MUL_D => "mul.d",
                DmJKOp::MUL_W => "mul.w",
                DmJKOp::MULH_D => "mulh.d",
                DmJKOp::MULH_DU => "mulh.du",
                DmJKOp::MULH_W => "mulh.w",
                DmJKOp::MULH_WU => "mulh.wu",
                DmJKOp::MULW_D_W => "mulw.d.w",
                DmJKOp::MULW_D_WU => "mulw.d.wu",
                DmJKOp::NOR => "nor",
                DmJKOp::OR => "or",
                DmJKOp::ORN => "orn",
                DmJKOp::ROTR_D => "rotr.d",
                DmJKOp::ROTR_W => "rotr.w",
                DmJKOp::SLL_D => "sll.d",
                DmJKOp::SLL_W => "sll.w",
                DmJKOp::SLT => "slt",
                DmJKOp::SLTU => "sltu",
                DmJKOp::SRA_D => "sra.d",
                DmJKOp::SRA_W => "sra.w",
                DmJKOp::SRL_D => "srl.d",
                DmJKOp::SRL_W => "srl.w",
                DmJKOp::SUB_D => "sub.d",
                DmJKOp::SUB_W => "sub.w",
                DmJKOp::XOR => "xor",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                reg_to_num(*rk)
            )?;
        }
        LInst::JSd5Sk16ps2 { op, rj, imm1, imm2 } => {
            f.write_str(match op {
                JSd5Sk16ps2Op::BEQZ => "beqz",
                JSd5Sk16ps2Op::BNEZ => "bnez",
            })?;
            write!(
                f,
                " $r{}, {}, {}",
                reg_to_num(*rj),
                imm1.as_i32(),
                imm2.as_i32()
            )?;
        }
        LInst::FdJSk12 { op, fd, rj, imm1 } => {
            f.write_str(match op {
                FdJSk12Op::FST_D => "fst.d",
                FdJSk12Op::FST_S => "fst.s",
            })?;
            write!(
                f,
                " $f{}, $r{}, {}",
                reg_to_num(*fd),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::DmSj20 { op, rd, imm1 } => {
            f.write_str(match op {
                DmSj20Op::LU12I_W => "lu12i.w",
                DmSj20Op::LU32I_D => "lu32i.d",
                DmSj20Op::PCADDI => "pcaddi",
                DmSj20Op::PCADDU12I => "pcaddu12i",
                DmSj20Op::PCADDU18I => "pcaddu18i",
                DmSj20Op::PCALAU12I => "pcalau12i",
            })?;
            write!(f, " $r{}, {}", reg_to_num(rd.to_reg()), imm1.as_i32())?;
        }
        LInst::FdmJK { op, fd, rj, rk } => {
            f.write_str(match op {
                FdmJKOp::FLDX_D => "fldx.d",
                FdmJKOp::FLDX_S => "fldx.s",
            })?;
            write!(
                f,
                " $f{}, $r{}, $r{}",
                reg_to_num(fd.to_reg()),
                reg_to_num(*rj),
                reg_to_num(*rk)
            )?;
        }
        LInst::DJSk14ps2 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DJSk14ps2Op::STPTR_D => "stptr.d",
                DJSk14ps2Op::STPTR_W => "stptr.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(*rd),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::DmJKUa2pp1 {
            op,
            rd,
            rj,
            rk,
            imm1,
        } => {
            f.write_str(match op {
                DmJKUa2pp1Op::ALSL_D => "alsl.d",
                DmJKUa2pp1Op::ALSL_W => "alsl.w",
                DmJKUa2pp1Op::ALSL_WU => "alsl.wu",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                reg_to_num(*rk),
                imm1.bits()
            )?;
        }
        LInst::DmJ { op, rd, rj } => {
            f.write_str(match op {
                DmJOp::BITREV_4B => "bitrev.4b",
                DmJOp::BITREV_8B => "bitrev.8b",
                DmJOp::BITREV_D => "bitrev.d",
                DmJOp::BITREV_W => "bitrev.w",
                DmJOp::CLO_D => "clo.d",
                DmJOp::CLO_W => "clo.w",
                DmJOp::CLZ_D => "clz.d",
                DmJOp::CLZ_W => "clz.w",
                DmJOp::CPUCFG => "cpucfg",
                DmJOp::CTO_D => "cto.d",
                DmJOp::CTO_W => "cto.w",
                DmJOp::CTZ_D => "ctz.d",
                DmJOp::CTZ_W => "ctz.w",
                DmJOp::EXT_W_B => "ext.w.b",
                DmJOp::EXT_W_H => "ext.w.h",
                DmJOp::LLACQ_D => "llacq.d",
                DmJOp::LLACQ_W => "llacq.w",
                DmJOp::MOVFCSR2GR => "movfcsr2gr",
                DmJOp::REVB_2H => "revb.2h",
                DmJOp::REVB_2W => "revb.2w",
                DmJOp::REVB_4H => "revb.4h",
                DmJOp::REVB_D => "revb.d",
                DmJOp::REVH_2W => "revh.2w",
                DmJOp::REVH_D => "revh.d",
                DmJOp::SCREL_D => "screl.d",
                DmJOp::SCREL_W => "screl.w",
            })?;
            write!(f, " $r{}, $r{}", reg_to_num(rd.to_reg()), reg_to_num(*rj))?;
        }
        LInst::FdmCj { op, fd, fcc_j } => {
            f.write_str(match op {
                FdmCjOp::MOVCF2FR => "movcf2fr",
            })?;
            write!(
                f,
                " $f{}, $fcc{}",
                reg_to_num(fd.to_reg()),
                reg_to_num(*fcc_j)
            )?;
        }
        LInst::CjSd5Sk16ps2 {
            op,
            fcc_j,
            imm1,
            imm2,
        } => {
            f.write_str(match op {
                CjSd5Sk16ps2Op::BCEQZ => "bceqz",
                CjSd5Sk16ps2Op::BCNEZ => "bcnez",
            })?;
            write!(
                f,
                " $fcc{}, {}, {}",
                reg_to_num(*fcc_j),
                imm1.as_i32(),
                imm2.as_i32()
            )?;
        }
        LInst::DmJUm5Uk5 {
            op,
            rd,
            rj,
            imm1,
            imm2,
        } => {
            f.write_str(match op {
                DmJUm5Uk5Op::BSTRINS_W => "bstrins.w",
                DmJUm5Uk5Op::BSTRPICK_W => "bstrpick.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.bits(),
                imm2.bits()
            )?;
        }
        LInst::DmJKUa2 {
            op,
            rd,
            rj,
            rk,
            imm1,
        } => {
            f.write_str(match op {
                DmJKUa2Op::BYTEPICK_W => "bytepick.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                reg_to_num(*rk),
                imm1.bits()
            )?;
        }
        LInst::DmKmJ { op, rd, rk, rj } => {
            f.write_str(match op {
                DmKmJOp::AMADD_B => "amadd.b",
                DmKmJOp::AMADD_D => "amadd.d",
                DmKmJOp::AMADD_H => "amadd.h",
                DmKmJOp::AMADD_W => "amadd.w",
                DmKmJOp::AMADD_DB_B => "amadd_db.b",
                DmKmJOp::AMADD_DB_H => "amadd_db.h",
                DmKmJOp::AMADD_DB_W => "amadd_db.w",
                DmKmJOp::AMSWAP_B => "amswap.b",
                DmKmJOp::AMSWAP_D => "amswap.d",
                DmKmJOp::AMSWAP_H => "amswap.h",
                DmKmJOp::AMSWAP_W => "amswap.w",
                DmKmJOp::AMSWAP_DB_B => "amswap_db.b",
                DmKmJOp::AMSWAP_DB_H => "amswap_db.h",
                DmKmJOp::AMSWAP_DB_W => "amswap_db.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}",
                reg_to_num(rd.to_reg()),
                reg_to_num(rk.to_reg()),
                reg_to_num(*rj)
            )?;
        }
        LInst::DmFj { op, rd, fj } => {
            f.write_str(match op {
                DmFjOp::MOVFR2GR_D => "movfr2gr.d",
                DmFjOp::MOVFR2GR_S => "movfr2gr.s",
                DmFjOp::MOVFRH2GR_S => "movfrh2gr.s",
            })?;
            write!(f, " $r{}, $f{}", reg_to_num(rd.to_reg()), reg_to_num(*fj))?;
        }
        LInst::FdmFj { op, fd, fj } => {
            f.write_str(match op {
                FdmFjOp::FABS_D => "fabs.d",
                FdmFjOp::FABS_S => "fabs.s",
                FdmFjOp::FCLASS_D => "fclass.d",
                FdmFjOp::FCLASS_S => "fclass.s",
                FdmFjOp::FCVT_D_S => "fcvt.d.s",
                FdmFjOp::FCVT_S_D => "fcvt.s.d",
                FdmFjOp::FFINT_D_L => "ffint.d.l",
                FdmFjOp::FFINT_D_W => "ffint.d.w",
                FdmFjOp::FFINT_S_L => "ffint.s.l",
                FdmFjOp::FFINT_S_W => "ffint.s.w",
                FdmFjOp::FLOGB_D => "flogb.d",
                FdmFjOp::FLOGB_S => "flogb.s",
                FdmFjOp::FMOV_D => "fmov.d",
                FdmFjOp::FMOV_S => "fmov.s",
                FdmFjOp::FNEG_D => "fneg.d",
                FdmFjOp::FNEG_S => "fneg.s",
                FdmFjOp::FRECIP_D => "frecip.d",
                FdmFjOp::FRECIP_S => "frecip.s",
                FdmFjOp::FRECIPE_D => "frecipe.d",
                FdmFjOp::FRECIPE_S => "frecipe.s",
                FdmFjOp::FRINT_D => "frint.d",
                FdmFjOp::FRINT_S => "frint.s",
                FdmFjOp::FRSQRT_D => "frsqrt.d",
                FdmFjOp::FRSQRT_S => "frsqrt.s",
                FdmFjOp::FRSQRTE_D => "frsqrte.d",
                FdmFjOp::FRSQRTE_S => "frsqrte.s",
                FdmFjOp::FSQRT_D => "fsqrt.d",
                FdmFjOp::FSQRT_S => "fsqrt.s",
                FdmFjOp::FTINT_L_D => "ftint.l.d",
                FdmFjOp::FTINT_L_S => "ftint.l.s",
                FdmFjOp::FTINT_W_D => "ftint.w.d",
                FdmFjOp::FTINT_W_S => "ftint.w.s",
                FdmFjOp::FTINTRM_L_D => "ftintrm.l.d",
                FdmFjOp::FTINTRM_L_S => "ftintrm.l.s",
                FdmFjOp::FTINTRM_W_D => "ftintrm.w.d",
                FdmFjOp::FTINTRM_W_S => "ftintrm.w.s",
                FdmFjOp::FTINTRNE_L_D => "ftintrne.l.d",
                FdmFjOp::FTINTRNE_L_S => "ftintrne.l.s",
                FdmFjOp::FTINTRNE_W_D => "ftintrne.w.d",
                FdmFjOp::FTINTRNE_W_S => "ftintrne.w.s",
                FdmFjOp::FTINTRP_L_D => "ftintrp.l.d",
                FdmFjOp::FTINTRP_L_S => "ftintrp.l.s",
                FdmFjOp::FTINTRP_W_D => "ftintrp.w.d",
                FdmFjOp::FTINTRP_W_S => "ftintrp.w.s",
                FdmFjOp::FTINTRZ_L_D => "ftintrz.l.d",
                FdmFjOp::FTINTRZ_L_S => "ftintrz.l.s",
                FdmFjOp::FTINTRZ_W_D => "ftintrz.w.d",
                FdmFjOp::FTINTRZ_W_S => "ftintrz.w.s",
            })?;
            write!(f, " $f{}, $f{}", reg_to_num(fd.to_reg()), reg_to_num(*fj))?;
        }
        LInst::DmJKUa3 {
            op,
            rd,
            rj,
            rk,
            imm1,
        } => {
            f.write_str(match op {
                DmJKUa3Op::BYTEPICK_D => "bytepick.d",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                reg_to_num(*rk),
                imm1.bits()
            )?;
        }
        LInst::DmJUk5 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJUk5Op::ROTRI_W => "rotri.w",
                DmJUk5Op::SLLI_W => "slli.w",
                DmJUk5Op::SRAI_W => "srai.w",
                DmJUk5Op::SRLI_W => "srli.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.bits()
            )?;
        }
        LInst::JK { op, rj, rk } => {
            f.write_str(match op {
                JKOp::ASRTGT_D => "asrtgt.d",
                JKOp::ASRTLE_D => "asrtle.d",
            })?;
            write!(f, " $r{}, $r{}", reg_to_num(*rj), reg_to_num(*rk))?;
        }
        LInst::DmJSk12 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJSk12Op::ADDI_D => "addi.d",
                DmJSk12Op::ADDI_W => "addi.w",
                DmJSk12Op::LD_B => "ld.b",
                DmJSk12Op::LD_BU => "ld.bu",
                DmJSk12Op::LD_D => "ld.d",
                DmJSk12Op::LD_H => "ld.h",
                DmJSk12Op::LD_HU => "ld.hu",
                DmJSk12Op::LD_W => "ld.w",
                DmJSk12Op::LD_WU => "ld.wu",
                DmJSk12Op::LU52I_D => "lu52i.d",
                DmJSk12Op::SLTI => "slti",
                DmJSk12Op::SLTUI => "sltui",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::DmJSk14ps2 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJSk14ps2Op::LDPTR_D => "ldptr.d",
                DmJSk14ps2Op::LDPTR_W => "ldptr.w",
                DmJSk14ps2Op::LL_D => "ll.d",
                DmJSk14ps2Op::LL_W => "ll.w",
                DmJSk14ps2Op::SC_D => "sc.d",
                DmJSk14ps2Op::SC_W => "sc.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::DJm { op, rd, rj } => {
            f.write_str(match op {
                DJmOp::MOVGR2FCSR => "movgr2fcsr",
            })?;
            write!(f, " $r{}, $r{}", reg_to_num(*rd), reg_to_num(rj.to_reg()))?;
        }
        LInst::CdmFj { op, fcc_d, fj } => {
            f.write_str(match op {
                CdmFjOp::MOVFR2CF => "movfr2cf",
            })?;
            write!(
                f,
                " $fcc{}, $f{}",
                reg_to_num(fcc_d.to_reg()),
                reg_to_num(*fj)
            )?;
        }
        LInst::JDSk16ps2 { op, rj, rd, imm1 } => {
            f.write_str(match op {
                JDSk16ps2Op::BEQ => "beq",
                JDSk16ps2Op::BGE => "bge",
                JDSk16ps2Op::BGEU => "bgeu",
                JDSk16ps2Op::BLT => "blt",
                JDSk16ps2Op::BLTU => "bltu",
                JDSk16ps2Op::BNE => "bne",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(*rj),
                reg_to_num(*rd),
                imm1.as_i32()
            )?;
        }
        LInst::DmJUk12 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJUk12Op::ANDI => "andi",
                DmJUk12Op::ORI => "ori",
                DmJUk12Op::XORI => "xori",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.bits()
            )?;
        }
        LInst::Ud15 { op, imm1 } => {
            f.write_str(match op {
                Ud15Op::BREAK => "break",
                Ud15Op::DBAR => "dbar",
                Ud15Op::DBCL => "dbcl",
                Ud15Op::IBAR => "ibar",
                Ud15Op::SYSCALL => "syscall",
            })?;
            write!(f, " {}", imm1.bits())?;
        }
        LInst::CdmFjFk { op, fcc_d, fj, fk } => {
            f.write_str(match op {
                CdmFjFkOp::FCMP_CAF_D => "fcmp.caf.d",
                CdmFjFkOp::FCMP_CAF_S => "fcmp.caf.s",
                CdmFjFkOp::FCMP_CEQ_D => "fcmp.ceq.d",
                CdmFjFkOp::FCMP_CEQ_S => "fcmp.ceq.s",
                CdmFjFkOp::FCMP_CLE_D => "fcmp.cle.d",
                CdmFjFkOp::FCMP_CLE_S => "fcmp.cle.s",
                CdmFjFkOp::FCMP_CLT_D => "fcmp.clt.d",
                CdmFjFkOp::FCMP_CLT_S => "fcmp.clt.s",
                CdmFjFkOp::FCMP_CNE_D => "fcmp.cne.d",
                CdmFjFkOp::FCMP_CNE_S => "fcmp.cne.s",
                CdmFjFkOp::FCMP_COR_D => "fcmp.cor.d",
                CdmFjFkOp::FCMP_COR_S => "fcmp.cor.s",
                CdmFjFkOp::FCMP_CUEQ_D => "fcmp.cueq.d",
                CdmFjFkOp::FCMP_CUEQ_S => "fcmp.cueq.s",
                CdmFjFkOp::FCMP_CULE_D => "fcmp.cule.d",
                CdmFjFkOp::FCMP_CULE_S => "fcmp.cule.s",
                CdmFjFkOp::FCMP_CULT_D => "fcmp.cult.d",
                CdmFjFkOp::FCMP_CULT_S => "fcmp.cult.s",
                CdmFjFkOp::FCMP_CUN_D => "fcmp.cun.d",
                CdmFjFkOp::FCMP_CUN_S => "fcmp.cun.s",
                CdmFjFkOp::FCMP_CUNE_D => "fcmp.cune.d",
                CdmFjFkOp::FCMP_CUNE_S => "fcmp.cune.s",
                CdmFjFkOp::FCMP_SAF_D => "fcmp.saf.d",
                CdmFjFkOp::FCMP_SAF_S => "fcmp.saf.s",
                CdmFjFkOp::FCMP_SEQ_D => "fcmp.seq.d",
                CdmFjFkOp::FCMP_SEQ_S => "fcmp.seq.s",
                CdmFjFkOp::FCMP_SLE_D => "fcmp.sle.d",
                CdmFjFkOp::FCMP_SLE_S => "fcmp.sle.s",
                CdmFjFkOp::FCMP_SLT_D => "fcmp.slt.d",
                CdmFjFkOp::FCMP_SLT_S => "fcmp.slt.s",
                CdmFjFkOp::FCMP_SNE_D => "fcmp.sne.d",
                CdmFjFkOp::FCMP_SNE_S => "fcmp.sne.s",
                CdmFjFkOp::FCMP_SOR_D => "fcmp.sor.d",
                CdmFjFkOp::FCMP_SOR_S => "fcmp.sor.s",
                CdmFjFkOp::FCMP_SUEQ_D => "fcmp.sueq.d",
                CdmFjFkOp::FCMP_SUEQ_S => "fcmp.sueq.s",
                CdmFjFkOp::FCMP_SULE_D => "fcmp.sule.d",
                CdmFjFkOp::FCMP_SULE_S => "fcmp.sule.s",
                CdmFjFkOp::FCMP_SULT_D => "fcmp.sult.d",
                CdmFjFkOp::FCMP_SULT_S => "fcmp.sult.s",
                CdmFjFkOp::FCMP_SUN_D => "fcmp.sun.d",
                CdmFjFkOp::FCMP_SUN_S => "fcmp.sun.s",
                CdmFjFkOp::FCMP_SUNE_D => "fcmp.sune.d",
                CdmFjFkOp::FCMP_SUNE_S => "fcmp.sune.s",
            })?;
            write!(
                f,
                " $fcc{}, $f{}, $f{}",
                reg_to_num(fcc_d.to_reg()),
                reg_to_num(*fj),
                reg_to_num(*fk)
            )?;
        }
        LInst::DJSk12 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DJSk12Op::ST_B => "st.b",
                DJSk12Op::ST_D => "st.d",
                DJSk12Op::ST_H => "st.h",
                DJSk12Op::ST_W => "st.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(*rd),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::DJK { op, rd, rj, rk } => {
            f.write_str(match op {
                DJKOp::STGT_B => "stgt.b",
                DJKOp::STGT_D => "stgt.d",
                DJKOp::STGT_H => "stgt.h",
                DJKOp::STGT_W => "stgt.w",
                DJKOp::STLE_B => "stle.b",
                DJKOp::STLE_D => "stle.d",
                DJKOp::STLE_H => "stle.h",
                DJKOp::STLE_W => "stle.w",
                DJKOp::STX_B => "stx.b",
                DJKOp::STX_D => "stx.d",
                DJKOp::STX_H => "stx.h",
                DJKOp::STX_W => "stx.w",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}",
                reg_to_num(*rd),
                reg_to_num(*rj),
                reg_to_num(*rk)
            )?;
        }
        LInst::FdmFjFkFa { op, fd, fj, fk, fa } => {
            f.write_str(match op {
                FdmFjFkFaOp::FMADD_D => "fmadd.d",
                FdmFjFkFaOp::FMADD_S => "fmadd.s",
                FdmFjFkFaOp::FMSUB_D => "fmsub.d",
                FdmFjFkFaOp::FMSUB_S => "fmsub.s",
                FdmFjFkFaOp::FNMADD_D => "fnmadd.d",
                FdmFjFkFaOp::FNMADD_S => "fnmadd.s",
                FdmFjFkFaOp::FNMSUB_D => "fnmsub.d",
                FdmFjFkFaOp::FNMSUB_S => "fnmsub.s",
            })?;
            write!(
                f,
                " $f{}, $f{}, $f{}, $f{}",
                reg_to_num(fd.to_reg()),
                reg_to_num(*fj),
                reg_to_num(*fk),
                reg_to_num(*fa)
            )?;
        }
        LInst::CdmJ { op, fcc_d, rj } => {
            f.write_str(match op {
                CdmJOp::MOVGR2CF => "movgr2cf",
            })?;
            write!(
                f,
                " $fcc{}, $r{}",
                reg_to_num(fcc_d.to_reg()),
                reg_to_num(*rj)
            )?;
        }
        LInst::DmJSk16 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJSk16Op::ADDU16I_D => "addu16i.d",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::DmKJ { op, rd, rk, rj } => {
            f.write_str(match op {
                DmKJOp::AMADD_DB_D => "amadd_db.d",
                DmKJOp::AMAND_D => "amand.d",
                DmKJOp::AMAND_W => "amand.w",
                DmKJOp::AMAND_DB_D => "amand_db.d",
                DmKJOp::AMAND_DB_W => "amand_db.w",
                DmKJOp::AMCAS_B => "amcas.b",
                DmKJOp::AMCAS_D => "amcas.d",
                DmKJOp::AMCAS_H => "amcas.h",
                DmKJOp::AMCAS_W => "amcas.w",
                DmKJOp::AMCAS_DB_B => "amcas_db.b",
                DmKJOp::AMCAS_DB_D => "amcas_db.d",
                DmKJOp::AMCAS_DB_H => "amcas_db.h",
                DmKJOp::AMCAS_DB_W => "amcas_db.w",
                DmKJOp::AMMAX_D => "ammax.d",
                DmKJOp::AMMAX_DU => "ammax.du",
                DmKJOp::AMMAX_W => "ammax.w",
                DmKJOp::AMMAX_WU => "ammax.wu",
                DmKJOp::AMMAX_DB_D => "ammax_db.d",
                DmKJOp::AMMAX_DB_DU => "ammax_db.du",
                DmKJOp::AMMAX_DB_W => "ammax_db.w",
                DmKJOp::AMMAX_DB_WU => "ammax_db.wu",
                DmKJOp::AMMIN_D => "ammin.d",
                DmKJOp::AMMIN_DU => "ammin.du",
                DmKJOp::AMMIN_W => "ammin.w",
                DmKJOp::AMMIN_WU => "ammin.wu",
                DmKJOp::AMMIN_DB_D => "ammin_db.d",
                DmKJOp::AMMIN_DB_DU => "ammin_db.du",
                DmKJOp::AMMIN_DB_W => "ammin_db.w",
                DmKJOp::AMMIN_DB_WU => "ammin_db.wu",
                DmKJOp::AMOR_D => "amor.d",
                DmKJOp::AMOR_W => "amor.w",
                DmKJOp::AMOR_DB_D => "amor_db.d",
                DmKJOp::AMOR_DB_W => "amor_db.w",
                DmKJOp::AMSWAP_DB_D => "amswap_db.d",
                DmKJOp::AMXOR_D => "amxor.d",
                DmKJOp::AMXOR_W => "amxor.w",
                DmKJOp::AMXOR_DB_D => "amxor_db.d",
                DmKJOp::AMXOR_DB_W => "amxor_db.w",
                DmKJOp::SC_Q => "sc.q",
            })?;
            write!(
                f,
                " $r{}, $r{}, $r{}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rk),
                reg_to_num(*rj)
            )?;
        }
        LInst::DmJUm6Uk6 {
            op,
            rd,
            rj,
            imm1,
            imm2,
        } => {
            f.write_str(match op {
                DmJUm6Uk6Op::BSTRINS_D => "bstrins.d",
                DmJUm6Uk6Op::BSTRPICK_D => "bstrpick.d",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.bits(),
                imm2.bits()
            )?;
        }
        LInst::Sd10Sk16ps2 { op, imm1, imm2 } => {
            f.write_str(match op {
                Sd10Sk16ps2Op::B => "b",
                Sd10Sk16ps2Op::BL => "bl",
            })?;
            write!(f, " {}, {}", imm1.as_i32(), imm2.as_i32())?;
        }
        LInst::FdmJ { op, fd, rj } => {
            f.write_str(match op {
                FdmJOp::MOVGR2FR_D => "movgr2fr.d",
                FdmJOp::MOVGR2FR_W => "movgr2fr.w",
                FdmJOp::MOVGR2FRH_W => "movgr2frh.w",
            })?;
            write!(f, " $f{}, $r{}", reg_to_num(fd.to_reg()), reg_to_num(*rj))?;
        }
        LInst::DmJm { op, rd, rj } => {
            f.write_str(match op {
                DmJmOp::RDTIME_D => "rdtime.d",
                DmJmOp::RDTIMEH_W => "rdtimeh.w",
                DmJmOp::RDTIMEL_W => "rdtimel.w",
            })?;
            write!(
                f,
                " $r{}, $r{}",
                reg_to_num(rd.to_reg()),
                reg_to_num(rj.to_reg())
            )?;
        }
        LInst::FdmJSk12 { op, fd, rj, imm1 } => {
            f.write_str(match op {
                FdmJSk12Op::FLD_D => "fld.d",
                FdmJSk12Op::FLD_S => "fld.s",
            })?;
            write!(
                f,
                " $f{}, $r{}, {}",
                reg_to_num(fd.to_reg()),
                reg_to_num(*rj),
                imm1.as_i32()
            )?;
        }
        LInst::Ud5JSk12 { op, imm1, rj, imm2 } => {
            f.write_str(match op {
                Ud5JSk12Op::PRELD => "preld",
            })?;
            write!(
                f,
                " {}, $r{}, {}",
                imm1.bits(),
                reg_to_num(*rj),
                imm2.as_i32()
            )?;
        }
        LInst::DmJUk6 { op, rd, rj, imm1 } => {
            f.write_str(match op {
                DmJUk6Op::ROTRI_D => "rotri.d",
                DmJUk6Op::SLLI_D => "slli.d",
                DmJUk6Op::SRAI_D => "srai.d",
                DmJUk6Op::SRLI_D => "srli.d",
            })?;
            write!(
                f,
                " $r{}, $r{}, {}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*rj),
                imm1.bits()
            )?;
        }
        LInst::DmCj { op, rd, fcc_j } => {
            f.write_str(match op {
                DmCjOp::MOVCF2GR => "movcf2gr",
            })?;
            write!(
                f,
                " $r{}, $fcc{}",
                reg_to_num(rd.to_reg()),
                reg_to_num(*fcc_j)
            )?;
        }
    }
    Ok(())
}
impl LInst {
    #[inline]
    pub fn enc_add_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::ADD_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_add_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::ADD_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_addi_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::ADDI_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_addi_w(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::ADDI_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_addu16i_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk16 {
            op: DmJSk16Op::ADDU16I_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_alsl_d(rd: WritableReg, rj: Reg, rk: Reg, imm1: u32) -> Self {
        Self::DmJKUa2pp1 {
            op: DmJKUa2pp1Op::ALSL_D,
            rd,
            rj,
            rk,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_alsl_w(rd: WritableReg, rj: Reg, rk: Reg, imm1: u32) -> Self {
        Self::DmJKUa2pp1 {
            op: DmJKUa2pp1Op::ALSL_W,
            rd,
            rj,
            rk,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_alsl_wu(rd: WritableReg, rj: Reg, rk: Reg, imm1: u32) -> Self {
        Self::DmJKUa2pp1 {
            op: DmJKUa2pp1Op::ALSL_WU,
            rd,
            rj,
            rk,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_amadd_b(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_B,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_d(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_h(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_H,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_w(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_db_b(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_DB_B,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMADD_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_db_h(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_DB_H,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amadd_db_w(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMADD_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amand_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMAND_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amand_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMAND_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amand_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMAND_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amand_db_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMAND_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_b(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_B,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_h(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_H,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_db_b(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_DB_B,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_db_h(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_DB_H,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amcas_db_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMCAS_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_du(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_DU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_wu(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_WU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_db_du(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_DB_DU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_db_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammax_db_wu(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMAX_DB_WU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_du(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_DU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_wu(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_WU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_db_du(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_DB_DU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_db_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_ammin_db_wu(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMMIN_DB_WU,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amor_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMOR_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amor_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMOR_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amor_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMOR_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amor_db_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMOR_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_b(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_B,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_d(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_h(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_H,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_w(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_db_b(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_DB_B,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMSWAP_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_db_h(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_DB_H,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amswap_db_w(rd: WritableReg, rk: WritableReg, rj: Reg) -> Self {
        Self::DmKmJ {
            op: DmKmJOp::AMSWAP_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amxor_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMXOR_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amxor_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMXOR_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amxor_db_d(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMXOR_DB_D,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_amxor_db_w(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::AMXOR_DB_W,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_and(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::AND,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_andi(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk12 {
            op: DmJUk12Op::ANDI,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_andn(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::ANDN,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_asrtgt_d(rj: Reg, rk: Reg) -> Self {
        Self::JK {
            op: JKOp::ASRTGT_D,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_asrtle_d(rj: Reg, rk: Reg) -> Self {
        Self::JK {
            op: JKOp::ASRTLE_D,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_b(imm1: i32, imm2: i32) -> Self {
        Self::Sd10Sk16ps2 {
            op: Sd10Sk16ps2Op::B,
            imm1: Simm::from_i32(imm1),
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_bceqz(fcc_j: Reg, imm1: i32, imm2: i32) -> Self {
        Self::CjSd5Sk16ps2 {
            op: CjSd5Sk16ps2Op::BCEQZ,
            fcc_j,
            imm1: Simm::from_i32(imm1),
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_bcnez(fcc_j: Reg, imm1: i32, imm2: i32) -> Self {
        Self::CjSd5Sk16ps2 {
            op: CjSd5Sk16ps2Op::BCNEZ,
            fcc_j,
            imm1: Simm::from_i32(imm1),
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_beq(rj: Reg, rd: Reg, imm1: i32) -> Self {
        Self::JDSk16ps2 {
            op: JDSk16ps2Op::BEQ,
            rj,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_beqz(rj: Reg, imm1: i32, imm2: i32) -> Self {
        Self::JSd5Sk16ps2 {
            op: JSd5Sk16ps2Op::BEQZ,
            rj,
            imm1: Simm::from_i32(imm1),
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_bge(rj: Reg, rd: Reg, imm1: i32) -> Self {
        Self::JDSk16ps2 {
            op: JDSk16ps2Op::BGE,
            rj,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_bgeu(rj: Reg, rd: Reg, imm1: i32) -> Self {
        Self::JDSk16ps2 {
            op: JDSk16ps2Op::BGEU,
            rj,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_bitrev_4b(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::BITREV_4B,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_bitrev_8b(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::BITREV_8B,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_bitrev_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::BITREV_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_bitrev_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::BITREV_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_bl(imm1: i32, imm2: i32) -> Self {
        Self::Sd10Sk16ps2 {
            op: Sd10Sk16ps2Op::BL,
            imm1: Simm::from_i32(imm1),
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_blt(rj: Reg, rd: Reg, imm1: i32) -> Self {
        Self::JDSk16ps2 {
            op: JDSk16ps2Op::BLT,
            rj,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_bltu(rj: Reg, rd: Reg, imm1: i32) -> Self {
        Self::JDSk16ps2 {
            op: JDSk16ps2Op::BLTU,
            rj,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_bne(rj: Reg, rd: Reg, imm1: i32) -> Self {
        Self::JDSk16ps2 {
            op: JDSk16ps2Op::BNE,
            rj,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_bnez(rj: Reg, imm1: i32, imm2: i32) -> Self {
        Self::JSd5Sk16ps2 {
            op: JSd5Sk16ps2Op::BNEZ,
            rj,
            imm1: Simm::from_i32(imm1),
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_break(imm1: u32) -> Self {
        Self::Ud15 {
            op: Ud15Op::BREAK,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_bstrins_d(rd: WritableReg, rj: Reg, imm1: u32, imm2: u32) -> Self {
        Self::DmJUm6Uk6 {
            op: DmJUm6Uk6Op::BSTRINS_D,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
            imm2: Uimm::from_u32(imm2),
        }
    }

    #[inline]
    pub fn enc_bstrins_w(rd: WritableReg, rj: Reg, imm1: u32, imm2: u32) -> Self {
        Self::DmJUm5Uk5 {
            op: DmJUm5Uk5Op::BSTRINS_W,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
            imm2: Uimm::from_u32(imm2),
        }
    }

    #[inline]
    pub fn enc_bstrpick_d(rd: WritableReg, rj: Reg, imm1: u32, imm2: u32) -> Self {
        Self::DmJUm6Uk6 {
            op: DmJUm6Uk6Op::BSTRPICK_D,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
            imm2: Uimm::from_u32(imm2),
        }
    }

    #[inline]
    pub fn enc_bstrpick_w(rd: WritableReg, rj: Reg, imm1: u32, imm2: u32) -> Self {
        Self::DmJUm5Uk5 {
            op: DmJUm5Uk5Op::BSTRPICK_W,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
            imm2: Uimm::from_u32(imm2),
        }
    }

    #[inline]
    pub fn enc_bytepick_d(rd: WritableReg, rj: Reg, rk: Reg, imm1: u32) -> Self {
        Self::DmJKUa3 {
            op: DmJKUa3Op::BYTEPICK_D,
            rd,
            rj,
            rk,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_bytepick_w(rd: WritableReg, rj: Reg, rk: Reg, imm1: u32) -> Self {
        Self::DmJKUa2 {
            op: DmJKUa2Op::BYTEPICK_W,
            rd,
            rj,
            rk,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_clo_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CLO_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_clo_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CLO_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_clz_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CLZ_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_clz_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CLZ_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_cpucfg(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CPUCFG,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_crc_w_b_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRC_W_B_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crc_w_d_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRC_W_D_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crc_w_h_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRC_W_H_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crc_w_w_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRC_W_W_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crcc_w_b_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRCC_W_B_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crcc_w_d_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRCC_W_D_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crcc_w_h_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRCC_W_H_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_crcc_w_w_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::CRCC_W_W_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_cto_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CTO_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_cto_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CTO_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_ctz_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CTZ_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_ctz_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::CTZ_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_dbar(imm1: u32) -> Self {
        Self::Ud15 {
            op: Ud15Op::DBAR,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_dbcl(imm1: u32) -> Self {
        Self::Ud15 {
            op: Ud15Op::DBCL,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_div_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::DIV_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_div_du(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::DIV_DU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_div_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::DIV_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_div_wu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::DIV_WU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ext_w_b(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::EXT_W_B,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_ext_w_h(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::EXT_W_H,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_fabs_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FABS_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fabs_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FABS_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fadd_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FADD_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fadd_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FADD_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fclass_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FCLASS_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fclass_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FCLASS_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fcmp_caf_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CAF_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_caf_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CAF_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_ceq_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CEQ_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_ceq_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CEQ_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cle_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CLE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cle_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CLE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_clt_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CLT_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_clt_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CLT_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cne_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CNE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cne_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CNE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cor_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_COR_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cor_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_COR_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cueq_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CUEQ_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cueq_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CUEQ_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cule_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CULE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cule_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CULE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cult_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CULT_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cult_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CULT_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cun_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CUN_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cun_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CUN_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cune_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CUNE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_cune_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_CUNE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_saf_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SAF_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_saf_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SAF_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_seq_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SEQ_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_seq_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SEQ_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sle_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SLE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sle_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SLE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_slt_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SLT_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_slt_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SLT_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sne_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SNE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sne_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SNE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sor_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SOR_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sor_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SOR_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sueq_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SUEQ_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sueq_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SUEQ_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sule_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SULE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sule_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SULE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sult_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SULT_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sult_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SULT_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sun_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SUN_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sun_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SUN_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sune_d(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SUNE_D,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcmp_sune_s(fcc_d: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::CdmFjFk {
            op: CdmFjFkOp::FCMP_SUNE_S,
            fcc_d,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcopysign_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FCOPYSIGN_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcopysign_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FCOPYSIGN_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fcvt_d_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FCVT_D_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fcvt_s_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FCVT_S_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fdiv_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FDIV_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fdiv_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FDIV_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_ffint_d_l(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FFINT_D_L,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ffint_d_w(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FFINT_D_W,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ffint_s_l(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FFINT_S_L,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ffint_s_w(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FFINT_S_W,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fld_d(fd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::FdmJSk12 {
            op: FdmJSk12Op::FLD_D,
            fd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_fld_s(fd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::FdmJSk12 {
            op: FdmJSk12Op::FLD_S,
            fd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_fldx_d(fd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::FdmJK {
            op: FdmJKOp::FLDX_D,
            fd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_fldx_s(fd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::FdmJK {
            op: FdmJKOp::FLDX_S,
            fd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_flogb_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FLOGB_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_flogb_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FLOGB_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fmadd_d(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FMADD_D,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fmadd_s(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FMADD_S,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fmax_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMAX_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmax_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMAX_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmaxa_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMAXA_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmaxa_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMAXA_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmin_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMIN_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmin_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMIN_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmina_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMINA_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmina_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMINA_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmov_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FMOV_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fmov_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FMOV_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fmsub_d(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FMSUB_D,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fmsub_s(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FMSUB_S,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fmul_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMUL_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fmul_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FMUL_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fneg_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FNEG_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fneg_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FNEG_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fnmadd_d(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FNMADD_D,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fnmadd_s(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FNMADD_S,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fnmsub_d(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FNMSUB_D,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_fnmsub_s(fd: WritableReg, fj: Reg, fk: Reg, fa: Reg) -> Self {
        Self::FdmFjFkFa {
            op: FdmFjFkFaOp::FNMSUB_S,
            fd,
            fj,
            fk,
            fa,
        }
    }

    #[inline]
    pub fn enc_frecip_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRECIP_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frecip_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRECIP_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frecipe_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRECIPE_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frecipe_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRECIPE_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frint_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRINT_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frint_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRINT_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frsqrt_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRSQRT_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frsqrt_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRSQRT_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frsqrte_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRSQRTE_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_frsqrte_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FRSQRTE_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fscaleb_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FSCALEB_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fscaleb_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FSCALEB_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fsel(fd: WritableReg, fj: Reg, fk: Reg, fcc_a: Reg) -> Self {
        Self::FdmFjFkCa {
            op: FdmFjFkCaOp::FSEL,
            fd,
            fj,
            fk,
            fcc_a,
        }
    }

    #[inline]
    pub fn enc_fsqrt_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FSQRT_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fsqrt_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FSQRT_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_fst_d(fd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::FdJSk12 {
            op: FdJSk12Op::FST_D,
            fd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_fst_s(fd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::FdJSk12 {
            op: FdJSk12Op::FST_S,
            fd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_fstx_d(fd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::FdJK {
            op: FdJKOp::FSTX_D,
            fd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_fstx_s(fd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::FdJK {
            op: FdJKOp::FSTX_S,
            fd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_fsub_d(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FSUB_D,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_fsub_s(fd: WritableReg, fj: Reg, fk: Reg) -> Self {
        Self::FdmFjFk {
            op: FdmFjFkOp::FSUB_S,
            fd,
            fj,
            fk,
        }
    }

    #[inline]
    pub fn enc_ftint_l_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINT_L_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftint_l_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINT_L_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftint_w_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINT_W_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftint_w_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINT_W_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrm_l_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRM_L_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrm_l_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRM_L_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrm_w_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRM_W_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrm_w_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRM_W_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrne_l_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRNE_L_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrne_l_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRNE_L_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrne_w_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRNE_W_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrne_w_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRNE_W_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrp_l_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRP_L_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrp_l_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRP_L_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrp_w_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRP_W_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrp_w_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRP_W_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrz_l_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRZ_L_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrz_l_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRZ_L_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrz_w_d(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRZ_W_D,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ftintrz_w_s(fd: WritableReg, fj: Reg) -> Self {
        Self::FdmFj {
            op: FdmFjOp::FTINTRZ_W_S,
            fd,
            fj,
        }
    }

    #[inline]
    pub fn enc_ibar(imm1: u32) -> Self {
        Self::Ud15 {
            op: Ud15Op::IBAR,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_jirl(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk16ps2 {
            op: DmJSk16ps2Op::JIRL,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_b(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_B,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_bu(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_BU,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_h(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_H,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_hu(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_HU,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_w(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ld_wu(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LD_WU,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ldgt_b(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDGT_B,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldgt_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDGT_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldgt_h(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDGT_H,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldgt_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDGT_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldle_b(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDLE_B,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldle_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDLE_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldle_h(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDLE_H,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldle_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDLE_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldptr_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk14ps2 {
            op: DmJSk14ps2Op::LDPTR_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ldptr_w(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk14ps2 {
            op: DmJSk14ps2Op::LDPTR_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ldx_b(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_B,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldx_bu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_BU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldx_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldx_h(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_H,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldx_hu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_HU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldx_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ldx_wu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::LDX_WU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ll_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk14ps2 {
            op: DmJSk14ps2Op::LL_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_ll_w(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk14ps2 {
            op: DmJSk14ps2Op::LL_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_llacq_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::LLACQ_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_llacq_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::LLACQ_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_lu12i_w(rd: WritableReg, imm1: i32) -> Self {
        Self::DmSj20 {
            op: DmSj20Op::LU12I_W,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_lu32i_d(rd: WritableReg, imm1: i32) -> Self {
        Self::DmSj20 {
            op: DmSj20Op::LU32I_D,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_lu52i_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::LU52I_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_maskeqz(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MASKEQZ,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_masknez(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MASKNEZ,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mod_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MOD_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mod_du(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MOD_DU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mod_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MOD_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mod_wu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MOD_WU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_movcf2fr(fd: WritableReg, fcc_j: Reg) -> Self {
        Self::FdmCj {
            op: FdmCjOp::MOVCF2FR,
            fd,
            fcc_j,
        }
    }

    #[inline]
    pub fn enc_movcf2gr(rd: WritableReg, fcc_j: Reg) -> Self {
        Self::DmCj {
            op: DmCjOp::MOVCF2GR,
            rd,
            fcc_j,
        }
    }

    #[inline]
    pub fn enc_movfcsr2gr(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::MOVFCSR2GR,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_movfr2cf(fcc_d: WritableReg, fj: Reg) -> Self {
        Self::CdmFj {
            op: CdmFjOp::MOVFR2CF,
            fcc_d,
            fj,
        }
    }

    #[inline]
    pub fn enc_movfr2gr_d(rd: WritableReg, fj: Reg) -> Self {
        Self::DmFj {
            op: DmFjOp::MOVFR2GR_D,
            rd,
            fj,
        }
    }

    #[inline]
    pub fn enc_movfr2gr_s(rd: WritableReg, fj: Reg) -> Self {
        Self::DmFj {
            op: DmFjOp::MOVFR2GR_S,
            rd,
            fj,
        }
    }

    #[inline]
    pub fn enc_movfrh2gr_s(rd: WritableReg, fj: Reg) -> Self {
        Self::DmFj {
            op: DmFjOp::MOVFRH2GR_S,
            rd,
            fj,
        }
    }

    #[inline]
    pub fn enc_movgr2cf(fcc_d: WritableReg, rj: Reg) -> Self {
        Self::CdmJ {
            op: CdmJOp::MOVGR2CF,
            fcc_d,
            rj,
        }
    }

    #[inline]
    pub fn enc_movgr2fcsr(rd: Reg, rj: WritableReg) -> Self {
        Self::DJm {
            op: DJmOp::MOVGR2FCSR,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_movgr2fr_d(fd: WritableReg, rj: Reg) -> Self {
        Self::FdmJ {
            op: FdmJOp::MOVGR2FR_D,
            fd,
            rj,
        }
    }

    #[inline]
    pub fn enc_movgr2fr_w(fd: WritableReg, rj: Reg) -> Self {
        Self::FdmJ {
            op: FdmJOp::MOVGR2FR_W,
            fd,
            rj,
        }
    }

    #[inline]
    pub fn enc_movgr2frh_w(fd: WritableReg, rj: Reg) -> Self {
        Self::FdmJ {
            op: FdmJOp::MOVGR2FRH_W,
            fd,
            rj,
        }
    }

    #[inline]
    pub fn enc_mul_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MUL_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mul_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MUL_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mulh_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MULH_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mulh_du(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MULH_DU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mulh_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MULH_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mulh_wu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MULH_WU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mulw_d_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MULW_D_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_mulw_d_wu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::MULW_D_WU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_nor(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::NOR,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_or(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::OR,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_ori(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk12 {
            op: DmJUk12Op::ORI,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_orn(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::ORN,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_pcaddi(rd: WritableReg, imm1: i32) -> Self {
        Self::DmSj20 {
            op: DmSj20Op::PCADDI,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_pcaddu12i(rd: WritableReg, imm1: i32) -> Self {
        Self::DmSj20 {
            op: DmSj20Op::PCADDU12I,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_pcaddu18i(rd: WritableReg, imm1: i32) -> Self {
        Self::DmSj20 {
            op: DmSj20Op::PCADDU18I,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_pcalau12i(rd: WritableReg, imm1: i32) -> Self {
        Self::DmSj20 {
            op: DmSj20Op::PCALAU12I,
            rd,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_preld(imm1: u32, rj: Reg, imm2: i32) -> Self {
        Self::Ud5JSk12 {
            op: Ud5JSk12Op::PRELD,
            imm1: Uimm::from_u32(imm1),
            rj,
            imm2: Simm::from_i32(imm2),
        }
    }

    #[inline]
    pub fn enc_preldx(imm1: u32, rj: Reg, rk: Reg) -> Self {
        Self::Ud5JK {
            op: Ud5JKOp::PRELDX,
            imm1: Uimm::from_u32(imm1),
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_rdtime_d(rd: WritableReg, rj: WritableReg) -> Self {
        Self::DmJm {
            op: DmJmOp::RDTIME_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_rdtimeh_w(rd: WritableReg, rj: WritableReg) -> Self {
        Self::DmJm {
            op: DmJmOp::RDTIMEH_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_rdtimel_w(rd: WritableReg, rj: WritableReg) -> Self {
        Self::DmJm {
            op: DmJmOp::RDTIMEL_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_revb_2h(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::REVB_2H,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_revb_2w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::REVB_2W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_revb_4h(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::REVB_4H,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_revb_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::REVB_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_revh_2w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::REVH_2W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_revh_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::REVH_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_rotr_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::ROTR_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_rotr_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::ROTR_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_rotri_d(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk6 {
            op: DmJUk6Op::ROTRI_D,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_rotri_w(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk5 {
            op: DmJUk5Op::ROTRI_W,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_sc_d(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk14ps2 {
            op: DmJSk14ps2Op::SC_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_sc_q(rd: WritableReg, rk: Reg, rj: Reg) -> Self {
        Self::DmKJ {
            op: DmKJOp::SC_Q,
            rd,
            rk,
            rj,
        }
    }

    #[inline]
    pub fn enc_sc_w(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk14ps2 {
            op: DmJSk14ps2Op::SC_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_screl_d(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::SCREL_D,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_screl_w(rd: WritableReg, rj: Reg) -> Self {
        Self::DmJ {
            op: DmJOp::SCREL_W,
            rd,
            rj,
        }
    }

    #[inline]
    pub fn enc_sll_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SLL_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_sll_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SLL_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_slli_d(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk6 {
            op: DmJUk6Op::SLLI_D,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_slli_w(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk5 {
            op: DmJUk5Op::SLLI_W,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_slt(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SLT,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_slti(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::SLTI,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_sltu(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SLTU,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_sltui(rd: WritableReg, rj: Reg, imm1: i32) -> Self {
        Self::DmJSk12 {
            op: DmJSk12Op::SLTUI,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_sra_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SRA_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_sra_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SRA_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_srai_d(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk6 {
            op: DmJUk6Op::SRAI_D,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_srai_w(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk5 {
            op: DmJUk5Op::SRAI_W,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_srl_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SRL_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_srl_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SRL_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_srli_d(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk6 {
            op: DmJUk6Op::SRLI_D,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_srli_w(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk5 {
            op: DmJUk5Op::SRLI_W,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_st_b(rd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::DJSk12 {
            op: DJSk12Op::ST_B,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_st_d(rd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::DJSk12 {
            op: DJSk12Op::ST_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_st_h(rd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::DJSk12 {
            op: DJSk12Op::ST_H,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_st_w(rd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::DJSk12 {
            op: DJSk12Op::ST_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_stgt_b(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STGT_B,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stgt_d(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STGT_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stgt_h(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STGT_H,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stgt_w(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STGT_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stle_b(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STLE_B,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stle_d(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STLE_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stle_h(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STLE_H,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stle_w(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STLE_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stptr_d(rd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::DJSk14ps2 {
            op: DJSk14ps2Op::STPTR_D,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_stptr_w(rd: Reg, rj: Reg, imm1: i32) -> Self {
        Self::DJSk14ps2 {
            op: DJSk14ps2Op::STPTR_W,
            rd,
            rj,
            imm1: Simm::from_i32(imm1),
        }
    }

    #[inline]
    pub fn enc_stx_b(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STX_B,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stx_d(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STX_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stx_h(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STX_H,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_stx_w(rd: Reg, rj: Reg, rk: Reg) -> Self {
        Self::DJK {
            op: DJKOp::STX_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_sub_d(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SUB_D,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_sub_w(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::SUB_W,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_syscall(imm1: u32) -> Self {
        Self::Ud15 {
            op: Ud15Op::SYSCALL,
            imm1: Uimm::from_u32(imm1),
        }
    }

    #[inline]
    pub fn enc_xor(rd: WritableReg, rj: Reg, rk: Reg) -> Self {
        Self::DmJK {
            op: DmJKOp::XOR,
            rd,
            rj,
            rk,
        }
    }

    #[inline]
    pub fn enc_xori(rd: WritableReg, rj: Reg, imm1: u32) -> Self {
        Self::DmJUk12 {
            op: DmJUk12Op::XORI,
            rd,
            rj,
            imm1: Uimm::from_u32(imm1),
        }
    }
}
