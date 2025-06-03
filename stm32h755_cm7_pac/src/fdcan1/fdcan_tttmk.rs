#[doc = "Register `FDCAN_TTTMK` reader"]
pub type R = crate::R<FdcanTttmkSpec>;
#[doc = "Register `FDCAN_TTTMK` writer"]
pub type W = crate::W<FdcanTttmkSpec>;
#[doc = "Field `TM` reader - Time Mark"]
pub type TmR = crate::FieldReader<u16>;
#[doc = "Field `TM` writer - Time Mark"]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TICC` reader - Time Mark Cycle Code"]
pub type TiccR = crate::FieldReader;
#[doc = "Field `TICC` writer - Time Mark Cycle Code"]
pub type TiccW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LCKM` reader - TT Time Mark Register Locked"]
pub type LckmR = crate::BitReader;
#[doc = "Field `LCKM` writer - TT Time Mark Register Locked"]
pub type LckmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&self) -> TiccR {
        TiccR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LckmR {
        LckmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<FdcanTttmkSpec> {
        TmW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&mut self) -> TiccW<FdcanTttmkSpec> {
        TiccW::new(self, 16)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&mut self) -> LckmW<FdcanTttmkSpec> {
        LckmW::new(self, 31)
    }
}
#[doc = "FDCAN TT Time Mark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tttmk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTttmkSpec;
impl crate::RegisterSpec for FdcanTttmkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tttmk::R`](R) reader structure"]
impl crate::Readable for FdcanTttmkSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tttmk::W`](W) writer structure"]
impl crate::Writable for FdcanTttmkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTTMK to value 0"]
impl crate::Resettable for FdcanTttmkSpec {}
