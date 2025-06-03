#[doc = "Register `L1WVPCR` reader"]
pub type R = crate::R<L1wvpcrSpec>;
#[doc = "Register `L1WVPCR` writer"]
pub type W = crate::W<L1wvpcrSpec>;
#[doc = "Field `WVSTPOS` reader - Window Vertical Start Position"]
pub type WvstposR = crate::FieldReader<u16>;
#[doc = "Field `WVSTPOS` writer - Window Vertical Start Position"]
pub type WvstposW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `WVSPPOS` reader - Window Vertical Stop Position"]
pub type WvspposR = crate::FieldReader<u16>;
#[doc = "Field `WVSPPOS` writer - Window Vertical Stop Position"]
pub type WvspposW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WvstposR {
        WvstposR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WvspposR {
        WvspposR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WvstposW<L1wvpcrSpec> {
        WvstposW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WvspposW<L1wvpcrSpec> {
        WvspposW::new(self, 16)
    }
}
#[doc = "Layerx Window Vertical Position Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1wvpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1wvpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1wvpcrSpec;
impl crate::RegisterSpec for L1wvpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1wvpcr::R`](R) reader structure"]
impl crate::Readable for L1wvpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1wvpcr::W`](W) writer structure"]
impl crate::Writable for L1wvpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1WVPCR to value 0"]
impl crate::Resettable for L1wvpcrSpec {}
