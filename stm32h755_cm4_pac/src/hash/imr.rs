#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `DINIE` reader - Data input interrupt enable"]
pub type DinieR = crate::BitReader;
#[doc = "Field `DINIE` writer - Data input interrupt enable"]
pub type DinieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIE` reader - Digest calculation completion interrupt enable"]
pub type DcieR = crate::BitReader;
#[doc = "Field `DCIE` writer - Digest calculation completion interrupt enable"]
pub type DcieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn dinie(&self) -> DinieR {
        DinieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn dcie(&self) -> DcieR {
        DcieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn dinie(&mut self) -> DinieW<ImrSpec> {
        DinieW::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn dcie(&mut self) -> DcieW<ImrSpec> {
        DcieW::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
