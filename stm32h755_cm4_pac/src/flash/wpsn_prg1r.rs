#[doc = "Register `WPSN_PRG1R` reader"]
pub type R = crate::R<WpsnPrg1rSpec>;
#[doc = "Register `WPSN_PRG1R` writer"]
pub type W = crate::W<WpsnPrg1rSpec>;
#[doc = "Field `WRPSn1` reader - Bank 1 sector write protection configuration byte"]
pub type Wrpsn1R = crate::FieldReader;
#[doc = "Field `WRPSn1` writer - Bank 1 sector write protection configuration byte"]
pub type Wrpsn1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection configuration byte"]
    #[inline(always)]
    pub fn wrpsn1(&self) -> Wrpsn1R {
        Wrpsn1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 1 sector write protection configuration byte"]
    #[inline(always)]
    pub fn wrpsn1(&mut self) -> Wrpsn1W<WpsnPrg1rSpec> {
        Wrpsn1W::new(self, 0)
    }
}
#[doc = "FLASH write sector protection for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_prg1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prg1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsnPrg1rSpec;
impl crate::RegisterSpec for WpsnPrg1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_prg1r::R`](R) reader structure"]
impl crate::Readable for WpsnPrg1rSpec {}
#[doc = "`write(|w| ..)` method takes [`wpsn_prg1r::W`](W) writer structure"]
impl crate::Writable for WpsnPrg1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPSN_PRG1R to value 0"]
impl crate::Resettable for WpsnPrg1rSpec {}
