#[doc = "Register `DMACCR` reader"]
pub type R = crate::R<DmaccrSpec>;
#[doc = "Register `DMACCR` writer"]
pub type W = crate::W<DmaccrSpec>;
#[doc = "Field `MSS` reader - Maximum Segment Size"]
pub type MssR = crate::FieldReader<u16>;
#[doc = "Field `MSS` writer - Maximum Segment Size"]
pub type MssW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PBLX8` reader - 8xPBL mode"]
pub type Pblx8R = crate::BitReader;
#[doc = "Field `PBLX8` writer - 8xPBL mode"]
pub type Pblx8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:13 - Maximum Segment Size"]
    #[inline(always)]
    pub fn mss(&self) -> MssR {
        MssR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 8xPBL mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> Pblx8R {
        Pblx8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Segment Size"]
    #[inline(always)]
    pub fn mss(&mut self) -> MssW<DmaccrSpec> {
        MssW::new(self, 0)
    }
    #[doc = "Bit 16 - 8xPBL mode"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> Pblx8W<DmaccrSpec> {
        Pblx8W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DslW<DmaccrSpec> {
        DslW::new(self, 18)
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaccrSpec;
impl crate::RegisterSpec for DmaccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccr::R`](R) reader structure"]
impl crate::Readable for DmaccrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaccr::W`](W) writer structure"]
impl crate::Writable for DmaccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACCR to value 0"]
impl crate::Resettable for DmaccrSpec {}
