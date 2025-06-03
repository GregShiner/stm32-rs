#[doc = "Register `IDC` reader"]
pub type R = crate::R<IdcSpec>;
#[doc = "Field `DEV_ID` reader - Device ID"]
pub type DevIdR = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - Revision"]
pub type RevIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Device ID"]
    #[inline(always)]
    pub fn dev_id(&self) -> DevIdR {
        DevIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision"]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU Identity Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcSpec;
impl crate::RegisterSpec for IdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idc::R`](R) reader structure"]
impl crate::Readable for IdcSpec {}
#[doc = "`reset()` method sets IDC to value 0x1000_6450"]
impl crate::Resettable for IdcSpec {
    const RESET_VALUE: u32 = 0x1000_6450;
}
