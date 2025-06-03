#[doc = "Register `FDCAN_GFC` reader"]
pub type R = crate::R<FdcanGfcSpec>;
#[doc = "Register `FDCAN_GFC` writer"]
pub type W = crate::W<FdcanGfcSpec>;
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RrfeR = crate::BitReader;
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RrfsR = crate::BitReader;
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type AnfeR = crate::FieldReader;
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type AnfsR = crate::FieldReader;
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RrfeW<FdcanGfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RrfsW<FdcanGfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&mut self) -> AnfeW<FdcanGfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&mut self) -> AnfsW<FdcanGfcSpec> {
        AnfsW::new(self, 4)
    }
}
#[doc = "FDCAN Global Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanGfcSpec;
impl crate::RegisterSpec for FdcanGfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_gfc::R`](R) reader structure"]
impl crate::Readable for FdcanGfcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_gfc::W`](W) writer structure"]
impl crate::Writable for FdcanGfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_GFC to value 0"]
impl crate::Resettable for FdcanGfcSpec {}
