#[doc = "Register `L2CR` reader"]
pub type R = crate::R<L2crSpec>;
#[doc = "Register `L2CR` writer"]
pub type W = crate::W<L2crSpec>;
#[doc = "Field `LEN` reader - Layer Enable"]
pub type LenR = crate::BitReader;
#[doc = "Field `LEN` writer - Layer Enable"]
pub type LenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLKEN` reader - Color Keying Enable"]
pub type ColkenR = crate::BitReader;
#[doc = "Field `COLKEN` writer - Color Keying Enable"]
pub type ColkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLUTEN` reader - Color Look-Up Table Enable"]
pub type ClutenR = crate::BitReader;
#[doc = "Field `CLUTEN` writer - Color Look-Up Table Enable"]
pub type ClutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    pub fn colken(&self) -> ColkenR {
        ColkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    pub fn cluten(&self) -> ClutenR {
        ClutenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<L2crSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    pub fn colken(&mut self) -> ColkenW<L2crSpec> {
        ColkenW::new(self, 1)
    }
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    pub fn cluten(&mut self) -> ClutenW<L2crSpec> {
        ClutenW::new(self, 4)
    }
}
#[doc = "Layerx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2crSpec;
impl crate::RegisterSpec for L2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2cr::R`](R) reader structure"]
impl crate::Readable for L2crSpec {}
#[doc = "`write(|w| ..)` method takes [`l2cr::W`](W) writer structure"]
impl crate::Writable for L2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2CR to value 0"]
impl crate::Resettable for L2crSpec {}
