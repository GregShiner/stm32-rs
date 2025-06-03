#[doc = "Register `FDCAN_TOCC` reader"]
pub type R = crate::R<FdcanToccSpec>;
#[doc = "Register `FDCAN_TOCC` writer"]
pub type W = crate::W<FdcanToccSpec>;
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type EtocR = crate::BitReader;
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TosR = crate::FieldReader;
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&mut self) -> EtocW<FdcanToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TosW<FdcanToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<FdcanToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanToccSpec;
impl crate::RegisterSpec for FdcanToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocc::R`](R) reader structure"]
impl crate::Readable for FdcanToccSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocc::W`](W) writer structure"]
impl crate::Writable for FdcanToccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TOCC to value 0"]
impl crate::Resettable for FdcanToccSpec {}
