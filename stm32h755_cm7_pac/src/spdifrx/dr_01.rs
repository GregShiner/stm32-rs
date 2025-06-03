#[doc = "Register `DR_01` reader"]
pub type R = crate::R<Dr01Spec>;
#[doc = "Field `PE` reader - Parity Error bit"]
pub type PeR = crate::BitReader;
#[doc = "Field `V` reader - Validity bit"]
pub type VR = crate::BitReader;
#[doc = "Field `U` reader - User bit"]
pub type UR = crate::BitReader;
#[doc = "Field `C` reader - Channel Status bit"]
pub type CR = crate::BitReader;
#[doc = "Field `PT` reader - Preamble Type"]
pub type PtR = crate::FieldReader;
#[doc = "Field `DR` reader - Data value"]
pub type DrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Data value"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr_01::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr01Spec;
impl crate::RegisterSpec for Dr01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr_01::R`](R) reader structure"]
impl crate::Readable for Dr01Spec {}
#[doc = "`reset()` method sets DR_01 to value 0"]
impl crate::Resettable for Dr01Spec {}
