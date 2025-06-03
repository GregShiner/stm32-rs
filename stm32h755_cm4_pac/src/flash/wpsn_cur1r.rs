#[doc = "Register `WPSN_CUR1R` reader"]
pub type R = crate::R<WpsnCur1rSpec>;
#[doc = "Field `WRPSn1` reader - Bank 1 sector write protection option status byte"]
pub type Wrpsn1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn1(&self) -> Wrpsn1R {
        Wrpsn1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_cur1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsnCur1rSpec;
impl crate::RegisterSpec for WpsnCur1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_cur1r::R`](R) reader structure"]
impl crate::Readable for WpsnCur1rSpec {}
#[doc = "`reset()` method sets WPSN_CUR1R to value 0"]
impl crate::Resettable for WpsnCur1rSpec {}
