#[doc = "Register `DR_00` reader"]
pub type R = crate::R<Dr00Spec>;
#[doc = "Field `DR` reader - Parity Error bit"]
pub type DrR = crate::FieldReader<u32>;
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
impl R {
    #[doc = "Bits 0:23 - Parity Error bit"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr_00::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr00Spec;
impl crate::RegisterSpec for Dr00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr_00::R`](R) reader structure"]
impl crate::Readable for Dr00Spec {}
#[doc = "`reset()` method sets DR_00 to value 0"]
impl crate::Resettable for Dr00Spec {}
