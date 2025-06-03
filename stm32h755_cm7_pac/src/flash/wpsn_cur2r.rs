#[doc = "Register `WPSN_CUR2R` reader"]
pub type R = crate::R<WpsnCur2rSpec>;
#[doc = "Field `WRPSn2` reader - Bank 2 sector write protection option status byte"]
pub type Wrpsn2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bank 2 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn2(&self) -> Wrpsn2R {
        Wrpsn2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_cur2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsnCur2rSpec;
impl crate::RegisterSpec for WpsnCur2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_cur2r::R`](R) reader structure"]
impl crate::Readable for WpsnCur2rSpec {}
#[doc = "`reset()` method sets WPSN_CUR2R to value 0"]
impl crate::Resettable for WpsnCur2rSpec {}
