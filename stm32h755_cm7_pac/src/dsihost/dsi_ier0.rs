#[doc = "Register `DSI_IER0` reader"]
pub type R = crate::R<DsiIer0Spec>;
#[doc = "Register `DSI_IER0` writer"]
pub type W = crate::W<DsiIer0Spec>;
#[doc = "Field `AE0IE` reader - AE0IE"]
pub type Ae0ieR = crate::BitReader;
#[doc = "Field `AE0IE` writer - AE0IE"]
pub type Ae0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE1IE` reader - AE1IE"]
pub type Ae1ieR = crate::BitReader;
#[doc = "Field `AE1IE` writer - AE1IE"]
pub type Ae1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE2IE` reader - AE2IE"]
pub type Ae2ieR = crate::BitReader;
#[doc = "Field `AE2IE` writer - AE2IE"]
pub type Ae2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE3IE` reader - AE3IE"]
pub type Ae3ieR = crate::BitReader;
#[doc = "Field `AE3IE` writer - AE3IE"]
pub type Ae3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE4IE` reader - AE4IE"]
pub type Ae4ieR = crate::BitReader;
#[doc = "Field `AE4IE` writer - AE4IE"]
pub type Ae4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE5IE` reader - AE5IE"]
pub type Ae5ieR = crate::BitReader;
#[doc = "Field `AE5IE` writer - AE5IE"]
pub type Ae5ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE6IE` reader - AE6IE"]
pub type Ae6ieR = crate::BitReader;
#[doc = "Field `AE6IE` writer - AE6IE"]
pub type Ae6ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE7IE` reader - AE7IE"]
pub type Ae7ieR = crate::BitReader;
#[doc = "Field `AE7IE` writer - AE7IE"]
pub type Ae7ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE8IE` reader - AE8IE"]
pub type Ae8ieR = crate::BitReader;
#[doc = "Field `AE8IE` writer - AE8IE"]
pub type Ae8ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE9IE` reader - AE9IE"]
pub type Ae9ieR = crate::BitReader;
#[doc = "Field `AE9IE` writer - AE9IE"]
pub type Ae9ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE10IE` reader - AE10IE"]
pub type Ae10ieR = crate::BitReader;
#[doc = "Field `AE10IE` writer - AE10IE"]
pub type Ae10ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE11IE` reader - AE11IE"]
pub type Ae11ieR = crate::BitReader;
#[doc = "Field `AE11IE` writer - AE11IE"]
pub type Ae11ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE12IE` reader - AE12IE"]
pub type Ae12ieR = crate::BitReader;
#[doc = "Field `AE12IE` writer - AE12IE"]
pub type Ae12ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE13IE` reader - AE13IE"]
pub type Ae13ieR = crate::BitReader;
#[doc = "Field `AE13IE` writer - AE13IE"]
pub type Ae13ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE14IE` reader - AE14IE"]
pub type Ae14ieR = crate::BitReader;
#[doc = "Field `AE14IE` writer - AE14IE"]
pub type Ae14ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE15IE` reader - AE15IE"]
pub type Ae15ieR = crate::BitReader;
#[doc = "Field `AE15IE` writer - AE15IE"]
pub type Ae15ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE0IE` reader - PE0IE"]
pub type Pe0ieR = crate::BitReader;
#[doc = "Field `PE0IE` writer - PE0IE"]
pub type Pe0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE1IE` reader - PE1IE"]
pub type Pe1ieR = crate::BitReader;
#[doc = "Field `PE1IE` writer - PE1IE"]
pub type Pe1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE2IE` reader - PE2IE"]
pub type Pe2ieR = crate::BitReader;
#[doc = "Field `PE2IE` writer - PE2IE"]
pub type Pe2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE3IE` reader - PE3IE"]
pub type Pe3ieR = crate::BitReader;
#[doc = "Field `PE3IE` writer - PE3IE"]
pub type Pe3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE4IE` reader - PE4IE"]
pub type Pe4ieR = crate::BitReader;
#[doc = "Field `PE4IE` writer - PE4IE"]
pub type Pe4ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AE0IE"]
    #[inline(always)]
    pub fn ae0ie(&self) -> Ae0ieR {
        Ae0ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AE1IE"]
    #[inline(always)]
    pub fn ae1ie(&self) -> Ae1ieR {
        Ae1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AE2IE"]
    #[inline(always)]
    pub fn ae2ie(&self) -> Ae2ieR {
        Ae2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AE3IE"]
    #[inline(always)]
    pub fn ae3ie(&self) -> Ae3ieR {
        Ae3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AE4IE"]
    #[inline(always)]
    pub fn ae4ie(&self) -> Ae4ieR {
        Ae4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AE5IE"]
    #[inline(always)]
    pub fn ae5ie(&self) -> Ae5ieR {
        Ae5ieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AE6IE"]
    #[inline(always)]
    pub fn ae6ie(&self) -> Ae6ieR {
        Ae6ieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AE7IE"]
    #[inline(always)]
    pub fn ae7ie(&self) -> Ae7ieR {
        Ae7ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AE8IE"]
    #[inline(always)]
    pub fn ae8ie(&self) -> Ae8ieR {
        Ae8ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AE9IE"]
    #[inline(always)]
    pub fn ae9ie(&self) -> Ae9ieR {
        Ae9ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AE10IE"]
    #[inline(always)]
    pub fn ae10ie(&self) -> Ae10ieR {
        Ae10ieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AE11IE"]
    #[inline(always)]
    pub fn ae11ie(&self) -> Ae11ieR {
        Ae11ieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AE12IE"]
    #[inline(always)]
    pub fn ae12ie(&self) -> Ae12ieR {
        Ae12ieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AE13IE"]
    #[inline(always)]
    pub fn ae13ie(&self) -> Ae13ieR {
        Ae13ieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AE14IE"]
    #[inline(always)]
    pub fn ae14ie(&self) -> Ae14ieR {
        Ae14ieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AE15IE"]
    #[inline(always)]
    pub fn ae15ie(&self) -> Ae15ieR {
        Ae15ieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PE0IE"]
    #[inline(always)]
    pub fn pe0ie(&self) -> Pe0ieR {
        Pe0ieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PE1IE"]
    #[inline(always)]
    pub fn pe1ie(&self) -> Pe1ieR {
        Pe1ieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PE2IE"]
    #[inline(always)]
    pub fn pe2ie(&self) -> Pe2ieR {
        Pe2ieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PE3IE"]
    #[inline(always)]
    pub fn pe3ie(&self) -> Pe3ieR {
        Pe3ieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PE4IE"]
    #[inline(always)]
    pub fn pe4ie(&self) -> Pe4ieR {
        Pe4ieR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AE0IE"]
    #[inline(always)]
    pub fn ae0ie(&mut self) -> Ae0ieW<DsiIer0Spec> {
        Ae0ieW::new(self, 0)
    }
    #[doc = "Bit 1 - AE1IE"]
    #[inline(always)]
    pub fn ae1ie(&mut self) -> Ae1ieW<DsiIer0Spec> {
        Ae1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - AE2IE"]
    #[inline(always)]
    pub fn ae2ie(&mut self) -> Ae2ieW<DsiIer0Spec> {
        Ae2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - AE3IE"]
    #[inline(always)]
    pub fn ae3ie(&mut self) -> Ae3ieW<DsiIer0Spec> {
        Ae3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - AE4IE"]
    #[inline(always)]
    pub fn ae4ie(&mut self) -> Ae4ieW<DsiIer0Spec> {
        Ae4ieW::new(self, 4)
    }
    #[doc = "Bit 5 - AE5IE"]
    #[inline(always)]
    pub fn ae5ie(&mut self) -> Ae5ieW<DsiIer0Spec> {
        Ae5ieW::new(self, 5)
    }
    #[doc = "Bit 6 - AE6IE"]
    #[inline(always)]
    pub fn ae6ie(&mut self) -> Ae6ieW<DsiIer0Spec> {
        Ae6ieW::new(self, 6)
    }
    #[doc = "Bit 7 - AE7IE"]
    #[inline(always)]
    pub fn ae7ie(&mut self) -> Ae7ieW<DsiIer0Spec> {
        Ae7ieW::new(self, 7)
    }
    #[doc = "Bit 8 - AE8IE"]
    #[inline(always)]
    pub fn ae8ie(&mut self) -> Ae8ieW<DsiIer0Spec> {
        Ae8ieW::new(self, 8)
    }
    #[doc = "Bit 9 - AE9IE"]
    #[inline(always)]
    pub fn ae9ie(&mut self) -> Ae9ieW<DsiIer0Spec> {
        Ae9ieW::new(self, 9)
    }
    #[doc = "Bit 10 - AE10IE"]
    #[inline(always)]
    pub fn ae10ie(&mut self) -> Ae10ieW<DsiIer0Spec> {
        Ae10ieW::new(self, 10)
    }
    #[doc = "Bit 11 - AE11IE"]
    #[inline(always)]
    pub fn ae11ie(&mut self) -> Ae11ieW<DsiIer0Spec> {
        Ae11ieW::new(self, 11)
    }
    #[doc = "Bit 12 - AE12IE"]
    #[inline(always)]
    pub fn ae12ie(&mut self) -> Ae12ieW<DsiIer0Spec> {
        Ae12ieW::new(self, 12)
    }
    #[doc = "Bit 13 - AE13IE"]
    #[inline(always)]
    pub fn ae13ie(&mut self) -> Ae13ieW<DsiIer0Spec> {
        Ae13ieW::new(self, 13)
    }
    #[doc = "Bit 14 - AE14IE"]
    #[inline(always)]
    pub fn ae14ie(&mut self) -> Ae14ieW<DsiIer0Spec> {
        Ae14ieW::new(self, 14)
    }
    #[doc = "Bit 15 - AE15IE"]
    #[inline(always)]
    pub fn ae15ie(&mut self) -> Ae15ieW<DsiIer0Spec> {
        Ae15ieW::new(self, 15)
    }
    #[doc = "Bit 16 - PE0IE"]
    #[inline(always)]
    pub fn pe0ie(&mut self) -> Pe0ieW<DsiIer0Spec> {
        Pe0ieW::new(self, 16)
    }
    #[doc = "Bit 17 - PE1IE"]
    #[inline(always)]
    pub fn pe1ie(&mut self) -> Pe1ieW<DsiIer0Spec> {
        Pe1ieW::new(self, 17)
    }
    #[doc = "Bit 18 - PE2IE"]
    #[inline(always)]
    pub fn pe2ie(&mut self) -> Pe2ieW<DsiIer0Spec> {
        Pe2ieW::new(self, 18)
    }
    #[doc = "Bit 19 - PE3IE"]
    #[inline(always)]
    pub fn pe3ie(&mut self) -> Pe3ieW<DsiIer0Spec> {
        Pe3ieW::new(self, 19)
    }
    #[doc = "Bit 20 - PE4IE"]
    #[inline(always)]
    pub fn pe4ie(&mut self) -> Pe4ieW<DsiIer0Spec> {
        Pe4ieW::new(self, 20)
    }
}
#[doc = "DSI Host interrupt enable register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIer0Spec;
impl crate::RegisterSpec for DsiIer0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ier0::R`](R) reader structure"]
impl crate::Readable for DsiIer0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_ier0::W`](W) writer structure"]
impl crate::Writable for DsiIer0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_IER0 to value 0"]
impl crate::Resettable for DsiIer0Spec {}
