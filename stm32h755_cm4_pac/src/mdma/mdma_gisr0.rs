#[doc = "Register `MDMA_GISR0` reader"]
pub type R = crate::R<MdmaGisr0Spec>;
#[doc = "Field `GIF0` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif0R = crate::BitReader;
#[doc = "Field `GIF1` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `GIF3` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `GIF4` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif4R = crate::BitReader;
#[doc = "Field `GIF5` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif5R = crate::BitReader;
#[doc = "Field `GIF6` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif6R = crate::BitReader;
#[doc = "Field `GIF7` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif7R = crate::BitReader;
#[doc = "Field `GIF8` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif8R = crate::BitReader;
#[doc = "Field `GIF9` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif9R = crate::BitReader;
#[doc = "Field `GIF10` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif10R = crate::BitReader;
#[doc = "Field `GIF11` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif11R = crate::BitReader;
#[doc = "Field `GIF12` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif12R = crate::BitReader;
#[doc = "Field `GIF13` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif13R = crate::BitReader;
#[doc = "Field `GIF14` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif14R = crate::BitReader;
#[doc = "Field `GIF15` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type Gif15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif0(&self) -> Gif0R {
        Gif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif7(&self) -> Gif7R {
        Gif7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif8(&self) -> Gif8R {
        Gif8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif9(&self) -> Gif9R {
        Gif9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif10(&self) -> Gif10R {
        Gif10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif11(&self) -> Gif11R {
        Gif11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif12(&self) -> Gif12R {
        Gif12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif13(&self) -> Gif13R {
        Gif13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif14(&self) -> Gif14R {
        Gif14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif15(&self) -> Gif15R {
        Gif15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "MDMA Global Interrupt/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_gisr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaGisr0Spec;
impl crate::RegisterSpec for MdmaGisr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_gisr0::R`](R) reader structure"]
impl crate::Readable for MdmaGisr0Spec {}
#[doc = "`reset()` method sets MDMA_GISR0 to value 0"]
impl crate::Resettable for MdmaGisr0Spec {}
