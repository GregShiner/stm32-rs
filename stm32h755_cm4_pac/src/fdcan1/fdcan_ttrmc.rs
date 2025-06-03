#[doc = "Register `FDCAN_TTRMC` reader"]
pub type R = crate::R<FdcanTtrmcSpec>;
#[doc = "Register `FDCAN_TTRMC` writer"]
pub type W = crate::W<FdcanTtrmcSpec>;
#[doc = "Field `RID` reader - Reference Identifier."]
pub type RidR = crate::FieldReader<u32>;
#[doc = "Field `RID` writer - Reference Identifier."]
pub type RidW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `XTD` reader - Extended Identifier"]
pub type XtdR = crate::BitReader;
#[doc = "Field `XTD` writer - Extended Identifier"]
pub type XtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMPS` reader - Reference Message Payload Select"]
pub type RmpsR = crate::BitReader;
#[doc = "Field `RMPS` writer - Reference Message Payload Select"]
pub type RmpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Reference Identifier."]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XtdR {
        XtdR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reference Message Payload Select"]
    #[inline(always)]
    pub fn rmps(&self) -> RmpsR {
        RmpsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Reference Identifier."]
    #[inline(always)]
    pub fn rid(&mut self) -> RidW<FdcanTtrmcSpec> {
        RidW::new(self, 0)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&mut self) -> XtdW<FdcanTtrmcSpec> {
        XtdW::new(self, 30)
    }
    #[doc = "Bit 31 - Reference Message Payload Select"]
    #[inline(always)]
    pub fn rmps(&mut self) -> RmpsW<FdcanTtrmcSpec> {
        RmpsW::new(self, 31)
    }
}
#[doc = "FDCAN TT Reference Message Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttrmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtrmcSpec;
impl crate::RegisterSpec for FdcanTtrmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttrmc::R`](R) reader structure"]
impl crate::Readable for FdcanTtrmcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttrmc::W`](W) writer structure"]
impl crate::Writable for FdcanTtrmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTRMC to value 0"]
impl crate::Resettable for FdcanTtrmcSpec {}
