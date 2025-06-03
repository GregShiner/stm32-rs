#[doc = "Register `MACECR` reader"]
pub type R = crate::R<MacecrSpec>;
#[doc = "Register `MACECR` writer"]
pub type W = crate::W<MacecrSpec>;
#[doc = "Field `GPSL` reader - GPSL"]
pub type GpslR = crate::FieldReader<u16>;
#[doc = "Field `GPSL` writer - GPSL"]
pub type GpslW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DCRCC` reader - DCRCC"]
pub type DcrccR = crate::BitReader;
#[doc = "Field `DCRCC` writer - DCRCC"]
pub type DcrccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEN` reader - SPEN"]
pub type SpenR = crate::BitReader;
#[doc = "Field `SPEN` writer - SPEN"]
pub type SpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USP` reader - USP"]
pub type UspR = crate::BitReader;
#[doc = "Field `USP` writer - USP"]
pub type UspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPGEN` reader - EIPGEN"]
pub type EipgenR = crate::BitReader;
#[doc = "Field `EIPGEN` writer - EIPGEN"]
pub type EipgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPG` reader - EIPG"]
pub type EipgR = crate::FieldReader;
#[doc = "Field `EIPG` writer - EIPG"]
pub type EipgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    pub fn gpsl(&self) -> GpslR {
        GpslR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DcrccR {
        DcrccR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    pub fn spen(&self) -> SpenR {
        SpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        UspR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    pub fn eipgen(&self) -> EipgenR {
        EipgenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    pub fn eipg(&self) -> EipgR {
        EipgR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    pub fn gpsl(&mut self) -> GpslW<MacecrSpec> {
        GpslW::new(self, 0)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    pub fn dcrcc(&mut self) -> DcrccW<MacecrSpec> {
        DcrccW::new(self, 16)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    pub fn spen(&mut self) -> SpenW<MacecrSpec> {
        SpenW::new(self, 17)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    pub fn usp(&mut self) -> UspW<MacecrSpec> {
        UspW::new(self, 18)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    pub fn eipgen(&mut self) -> EipgenW<MacecrSpec> {
        EipgenW::new(self, 24)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    pub fn eipg(&mut self) -> EipgW<MacecrSpec> {
        EipgW::new(self, 25)
    }
}
#[doc = "Extended operating mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`macecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacecrSpec;
impl crate::RegisterSpec for MacecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macecr::R`](R) reader structure"]
impl crate::Readable for MacecrSpec {}
#[doc = "`write(|w| ..)` method takes [`macecr::W`](W) writer structure"]
impl crate::Writable for MacecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACECR to value 0"]
impl crate::Resettable for MacecrSpec {}
