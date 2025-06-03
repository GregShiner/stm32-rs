#[doc = "Register `MACLTCR` reader"]
pub type R = crate::R<MacltcrSpec>;
#[doc = "Register `MACLTCR` writer"]
pub type W = crate::W<MacltcrSpec>;
#[doc = "Field `TWT` reader - TWT"]
pub type TwtR = crate::FieldReader<u16>;
#[doc = "Field `TWT` writer - TWT"]
pub type TwtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LST` reader - LST"]
pub type LstR = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - LST"]
pub type LstW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&self) -> TwtR {
        TwtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&self) -> LstR {
        LstR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&mut self) -> TwtW<MacltcrSpec> {
        TwtW::new(self, 0)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&mut self) -> LstW<MacltcrSpec> {
        LstW::new(self, 16)
    }
}
#[doc = "LPI timers control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacltcrSpec;
impl crate::RegisterSpec for MacltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macltcr::R`](R) reader structure"]
impl crate::Readable for MacltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`macltcr::W`](W) writer structure"]
impl crate::Writable for MacltcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for MacltcrSpec {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
