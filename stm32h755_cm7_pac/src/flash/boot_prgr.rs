#[doc = "Register `BOOT_PRGR` reader"]
pub type R = crate::R<BootPrgrSpec>;
#[doc = "Field `BOOT_ADD0` reader - Boot address 0"]
pub type BootAdd0R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_ADD1` reader - Boot address 1"]
pub type BootAdd1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BootAdd0R {
        BootAdd0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BootAdd1R {
        BootAdd1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FLASH register with boot address\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_prgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootPrgrSpec;
impl crate::RegisterSpec for BootPrgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_prgr::R`](R) reader structure"]
impl crate::Readable for BootPrgrSpec {}
#[doc = "`reset()` method sets BOOT_PRGR to value 0"]
impl crate::Resettable for BootPrgrSpec {}
