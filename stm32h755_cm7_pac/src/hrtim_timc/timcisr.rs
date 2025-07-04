#[doc = "Register `TIMCISR` reader"]
pub type R = crate::R<TimcisrSpec>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Flag"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP2` reader - Compare 2 Interrupt Flag"]
pub type Cmp2R = crate::BitReader;
#[doc = "Field `CMP3` reader - Compare 3 Interrupt Flag"]
pub type Cmp3R = crate::BitReader;
#[doc = "Field `CMP4` reader - Compare 4 Interrupt Flag"]
pub type Cmp4R = crate::BitReader;
#[doc = "Field `REP` reader - Repetition Interrupt Flag"]
pub type RepR = crate::BitReader;
#[doc = "Field `UPD` reader - Update Interrupt Flag"]
pub type UpdR = crate::BitReader;
#[doc = "Field `CPT1` reader - Capture1 Interrupt Flag"]
pub type Cpt1R = crate::BitReader;
#[doc = "Field `CPT2` reader - Capture2 Interrupt Flag"]
pub type Cpt2R = crate::BitReader;
#[doc = "Field `SETx1` reader - Output 1 Set Interrupt Flag"]
pub type Setx1R = crate::BitReader;
#[doc = "Field `RSTx1` reader - Output 1 Reset Interrupt Flag"]
pub type Rstx1R = crate::BitReader;
#[doc = "Field `SETx2` reader - Output 2 Set Interrupt Flag"]
pub type Setx2R = crate::BitReader;
#[doc = "Field `RSTx2` reader - Output 2 Reset Interrupt Flag"]
pub type Rstx2R = crate::BitReader;
#[doc = "Field `RST` reader - Reset Interrupt Flag"]
pub type RstR = crate::BitReader;
#[doc = "Field `DLYPRT` reader - Delayed Protection Flag"]
pub type DlyprtR = crate::BitReader;
#[doc = "Field `CPPSTAT` reader - Current Push Pull Status"]
pub type CppstatR = crate::BitReader;
#[doc = "Field `IPPSTAT` reader - Idle Push Pull Status"]
pub type IppstatR = crate::BitReader;
#[doc = "Field `O1STAT` reader - Output 1 State"]
pub type O1statR = crate::BitReader;
#[doc = "Field `O2STAT` reader - Output 2 State"]
pub type O2statR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp2(&self) -> Cmp2R {
        Cmp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp3(&self) -> Cmp3R {
        Cmp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp4(&self) -> Cmp4R {
        Cmp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Update Interrupt Flag"]
    #[inline(always)]
    pub fn upd(&self) -> UpdR {
        UpdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture1 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt1(&self) -> Cpt1R {
        Cpt1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture2 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt2(&self) -> Cpt2R {
        Cpt2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output 1 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx1(&self) -> Setx1R {
        Setx1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output 1 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx1(&self) -> Rstx1R {
        Rstx1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output 2 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx2(&self) -> Setx2R {
        Setx2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output 2 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx2(&self) -> Rstx2R {
        Rstx2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed Protection Flag"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DlyprtR {
        DlyprtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Current Push Pull Status"]
    #[inline(always)]
    pub fn cppstat(&self) -> CppstatR {
        CppstatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Idle Push Pull Status"]
    #[inline(always)]
    pub fn ippstat(&self) -> IppstatR {
        IppstatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 1 State"]
    #[inline(always)]
    pub fn o1stat(&self) -> O1statR {
        O1statR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 State"]
    #[inline(always)]
    pub fn o2stat(&self) -> O2statR {
        O2statR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Timerx Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timcisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimcisrSpec;
impl crate::RegisterSpec for TimcisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timcisr::R`](R) reader structure"]
impl crate::Readable for TimcisrSpec {}
#[doc = "`reset()` method sets TIMCISR to value 0"]
impl crate::Resettable for TimcisrSpec {}
