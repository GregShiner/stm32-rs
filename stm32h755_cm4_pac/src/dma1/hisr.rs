#[doc = "Register `HISR` reader"]
pub type R = crate::R<HisrSpec>;
#[doc = "Field `FEIF4` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feif4R = crate::BitReader;
#[doc = "Field `DMEIF4` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeif4R = crate::BitReader;
#[doc = "Field `TEIF4` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Teif4R = crate::BitReader;
#[doc = "Field `HTIF4` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htif4R = crate::BitReader;
#[doc = "Field `TCIF4` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Tcif4R = crate::BitReader;
#[doc = "Field `FEIF5` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feif5R = crate::BitReader;
#[doc = "Field `DMEIF5` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeif5R = crate::BitReader;
#[doc = "Field `TEIF5` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Teif5R = crate::BitReader;
#[doc = "Field `HTIF5` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htif5R = crate::BitReader;
#[doc = "Field `TCIF5` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Tcif5R = crate::BitReader;
#[doc = "Field `FEIF6` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feif6R = crate::BitReader;
#[doc = "Field `DMEIF6` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeif6R = crate::BitReader;
#[doc = "Field `TEIF6` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Teif6R = crate::BitReader;
#[doc = "Field `HTIF6` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htif6R = crate::BitReader;
#[doc = "Field `TCIF6` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Tcif6R = crate::BitReader;
#[doc = "Field `FEIF7` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feif7R = crate::BitReader;
#[doc = "Field `DMEIF7` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeif7R = crate::BitReader;
#[doc = "Field `TEIF7` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Teif7R = crate::BitReader;
#[doc = "Field `HTIF7` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htif7R = crate::BitReader;
#[doc = "Field `TCIF7` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Tcif7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif4(&self) -> Feif4R {
        Feif4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif4(&self) -> Dmeif4R {
        Dmeif4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif5(&self) -> Feif5R {
        Feif5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif5(&self) -> Dmeif5R {
        Dmeif5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif6(&self) -> Feif6R {
        Feif6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif6(&self) -> Dmeif6R {
        Dmeif6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        Htif6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif7(&self) -> Feif7R {
        Feif7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif7(&self) -> Dmeif7R {
        Dmeif7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        Htif7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "high interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HisrSpec;
impl crate::RegisterSpec for HisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hisr::R`](R) reader structure"]
impl crate::Readable for HisrSpec {}
#[doc = "`reset()` method sets HISR to value 0"]
impl crate::Resettable for HisrSpec {}
