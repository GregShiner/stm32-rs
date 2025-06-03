#[doc = "Register `L2BFCR` reader"]
pub type R = crate::R<L2bfcrSpec>;
#[doc = "Register `L2BFCR` writer"]
pub type W = crate::W<L2bfcrSpec>;
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
    pub fn bf2(&mut self) -> Bf2W<L2bfcrSpec> {
        Bf2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&mut self) -> Bf1W<L2bfcrSpec> {
        Bf1W::new(self, 8)
    }
}
#[doc = "Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2bfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2bfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2bfcrSpec;
impl crate::RegisterSpec for L2bfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2bfcr::R`](R) reader structure"]
impl crate::Readable for L2bfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`l2bfcr::W`](W) writer structure"]
impl crate::Writable for L2bfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2BFCR to value 0x0607"]
impl crate::Resettable for L2bfcrSpec {
    const RESET_VALUE: u32 = 0x0607;
}
