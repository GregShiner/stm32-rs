#[doc = "Register `FDCAN_ECR` reader"]
pub type R = crate::R<FdcanEcrSpec>;
#[doc = "Register `FDCAN_ECR` writer"]
pub type W = crate::W<FdcanEcrSpec>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub type TecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TREC` reader - Receive Error Counter"]
pub type TrecR = crate::FieldReader;
#[doc = "Field `TREC` writer - Receive Error Counter"]
pub type TrecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RP` reader - Receive Error Passive"]
pub type RpR = crate::BitReader;
#[doc = "Field `RP` writer - Receive Error Passive"]
pub type RpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEL` reader - AN Error Logging"]
pub type CelR = crate::FieldReader;
#[doc = "Field `CEL` writer - AN Error Logging"]
pub type CelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn trec(&self) -> TrecR {
        TrecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CelR {
        CelR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&mut self) -> TecW<FdcanEcrSpec> {
        TecW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn trec(&mut self) -> TrecW<FdcanEcrSpec> {
        TrecW::new(self, 8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&mut self) -> RpW<FdcanEcrSpec> {
        RpW::new(self, 15)
    }
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    pub fn cel(&mut self) -> CelW<FdcanEcrSpec> {
        CelW::new(self, 16)
    }
}
#[doc = "FDCAN Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanEcrSpec;
impl crate::RegisterSpec for FdcanEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ecr::R`](R) reader structure"]
impl crate::Readable for FdcanEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure"]
impl crate::Writable for FdcanEcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_ECR to value 0"]
impl crate::Resettable for FdcanEcrSpec {}
