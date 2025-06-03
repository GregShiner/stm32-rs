#[doc = "Register `L1BFCR` reader"]
pub type R = crate::R<L1bfcrSpec>;
#[doc = "Register `L1BFCR` writer"]
pub type W = crate::W<L1bfcrSpec>;
#[doc = "Field `BF2` reader - Blending Factor 2"]
pub type Bf2R = crate::FieldReader;
#[doc = "Field `BF2` writer - Blending Factor 2"]
pub type Bf2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BF1` reader - Blending Factor 1"]
pub type Bf1R = crate::FieldReader;
#[doc = "Field `BF1` writer - Blending Factor 1"]
pub type Bf1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> Bf2R {
        Bf2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> Bf1R {
        Bf1R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&mut self) -> Bf2W<L1bfcrSpec> {
        Bf2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&mut self) -> Bf1W<L1bfcrSpec> {
        Bf1W::new(self, 8)
    }
}
#[doc = "Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1bfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1bfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1bfcrSpec;
impl crate::RegisterSpec for L1bfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1bfcr::R`](R) reader structure"]
impl crate::Readable for L1bfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1bfcr::W`](W) writer structure"]
impl crate::Writable for L1bfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1BFCR to value 0x0607"]
impl crate::Resettable for L1bfcrSpec {
    const RESET_VALUE: u32 = 0x0607;
}
