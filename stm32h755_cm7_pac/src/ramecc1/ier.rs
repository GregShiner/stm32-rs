#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `GIE` reader - Global interrupt enable"]
pub type GieR = crate::BitReader;
#[doc = "Field `GIE` writer - Global interrupt enable"]
pub type GieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GECCSEIE_` reader - Global ECC single error interrupt enable"]
pub type Geccseie_R = crate::BitReader;
#[doc = "Field `GECCSEIE_` writer - Global ECC single error interrupt enable"]
pub type Geccseie_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GECCDEIE` reader - Global ECC double error interrupt enable"]
pub type GeccdeieR = crate::BitReader;
#[doc = "Field `GECCDEIE` writer - Global ECC double error interrupt enable"]
pub type GeccdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GECCDEBWIE` reader - Global ECC double error on byte write (BW) interrupt enable"]
pub type GeccdebwieR = crate::BitReader;
#[doc = "Field `GECCDEBWIE` writer - Global ECC double error on byte write (BW) interrupt enable"]
pub type GeccdebwieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GieR {
        GieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie_(&self) -> Geccseie_R {
        Geccseie_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&self) -> GeccdeieR {
        GeccdeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&self) -> GeccdebwieR {
        GeccdebwieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&mut self) -> GieW<IerSpec> {
        GieW::new(self, 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie_(&mut self) -> Geccseie_W<IerSpec> {
        Geccseie_W::new(self, 1)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&mut self) -> GeccdeieW<IerSpec> {
        GeccdeieW::new(self, 2)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&mut self) -> GeccdebwieW<IerSpec> {
        GeccdebwieW::new(self, 3)
    }
}
#[doc = "RAMECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
