#[doc = "Register `WPSN_PRG2R` reader"]
pub type R = crate::R<WpsnPrg2rSpec>;
#[doc = "Register `WPSN_PRG2R` writer"]
pub type W = crate::W<WpsnPrg2rSpec>;
#[doc = "Field `WRPSn2` reader - Bank 2 sector write protection configuration byte"]
pub type Wrpsn2R = crate::FieldReader;
#[doc = "Field `WRPSn2` writer - Bank 2 sector write protection configuration byte"]
pub type Wrpsn2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 sector write protection configuration byte"]
    #[inline(always)]
    pub fn wrpsn2(&self) -> Wrpsn2R {
        Wrpsn2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 sector write protection configuration byte"]
    #[inline(always)]
    pub fn wrpsn2(&mut self) -> Wrpsn2W<WpsnPrg2rSpec> {
        Wrpsn2W::new(self, 0)
    }
}
#[doc = "FLASH write sector protection for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_prg2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prg2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsnPrg2rSpec;
impl crate::RegisterSpec for WpsnPrg2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_prg2r::R`](R) reader structure"]
impl crate::Readable for WpsnPrg2rSpec {}
#[doc = "`write(|w| ..)` method takes [`wpsn_prg2r::W`](W) writer structure"]
impl crate::Writable for WpsnPrg2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPSN_PRG2R to value 0"]
impl crate::Resettable for WpsnPrg2rSpec {}
