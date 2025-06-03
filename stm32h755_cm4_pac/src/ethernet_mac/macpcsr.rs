#[doc = "Register `MACPCSR` reader"]
pub type R = crate::R<MacpcsrSpec>;
#[doc = "Register `MACPCSR` writer"]
pub type W = crate::W<MacpcsrSpec>;
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub type PwrdwnR = crate::BitReader;
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub type PwrdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPKTEN` reader - MGKPKTEN"]
pub type MgkpktenR = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - MGKPKTEN"]
pub type MgkpktenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPKTEN` reader - RWKPKTEN"]
pub type RwkpktenR = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - RWKPKTEN"]
pub type RwkpktenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPRCVD` reader - MGKPRCVD"]
pub type MgkprcvdR = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - RWKPRCVD"]
pub type RwkprcvdR = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - GLBLUCAST"]
pub type GlblucastR = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - GLBLUCAST"]
pub type GlblucastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPFE` reader - RWKPFE"]
pub type RwkpfeR = crate::BitReader;
#[doc = "Field `RWKPFE` writer - RWKPFE"]
pub type RwkpfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPTR` reader - RWKPTR"]
pub type RwkptrR = crate::FieldReader;
#[doc = "Field `RWKPTR` writer - RWKPTR"]
pub type RwkptrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RWKFILTRST` reader - RWKFILTRST"]
pub type RwkfiltrstR = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - RWKFILTRST"]
pub type RwkfiltrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PwrdwnR {
        PwrdwnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MgkpktenR {
        MgkpktenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RwkpktenR {
        RwkpktenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - MGKPRCVD"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MgkprcvdR {
        MgkprcvdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWKPRCVD"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RwkprcvdR {
        RwkprcvdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&self) -> GlblucastR {
        GlblucastR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RwkpfeR {
        RwkpfeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RwkptrR {
        RwkptrR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RwkfiltrstR {
        RwkfiltrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PwrdwnW<MacpcsrSpec> {
        PwrdwnW::new(self, 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MgkpktenW<MacpcsrSpec> {
        MgkpktenW::new(self, 1)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RwkpktenW<MacpcsrSpec> {
        RwkpktenW::new(self, 2)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GlblucastW<MacpcsrSpec> {
        GlblucastW::new(self, 9)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RwkpfeW<MacpcsrSpec> {
        RwkpfeW::new(self, 10)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> RwkptrW<MacpcsrSpec> {
        RwkptrW::new(self, 24)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RwkfiltrstW<MacpcsrSpec> {
        RwkfiltrstW::new(self, 31)
    }
}
#[doc = "PMT control status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacpcsrSpec;
impl crate::RegisterSpec for MacpcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpcsr::R`](R) reader structure"]
impl crate::Readable for MacpcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`macpcsr::W`](W) writer structure"]
impl crate::Writable for MacpcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPCSR to value 0"]
impl crate::Resettable for MacpcsrSpec {}
