#[doc = "Register `C7CR` reader"]
pub type R = crate::R<C7crSpec>;
#[doc = "Register `C7CR` writer"]
pub type W = crate::W<C7crSpec>;
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub type DmareqIdR = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub type DmareqIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub type SoieR = crate::BitReader;
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub type SoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub type EgeR = crate::BitReader;
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub type EgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub type SeR = crate::BitReader;
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SpolR = crate::FieldReader;
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NbreqR = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NbreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub type SyncIdR = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub type SyncIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DmareqIdR {
        DmareqIdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&self) -> SoieR {
        SoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EgeR {
        EgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SpolR {
        SpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&self) -> NbreqR {
        NbreqR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&self) -> SyncIdR {
        SyncIdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DmareqIdW<C7crSpec> {
        DmareqIdW::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&mut self) -> SoieW<C7crSpec> {
        SoieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&mut self) -> EgeW<C7crSpec> {
        EgeW::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&mut self) -> SeW<C7crSpec> {
        SeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&mut self) -> SpolW<C7crSpec> {
        SpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&mut self) -> NbreqW<C7crSpec> {
        NbreqW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&mut self) -> SyncIdW<C7crSpec> {
        SyncIdW::new(self, 24)
    }
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C7crSpec;
impl crate::RegisterSpec for C7crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c7cr::R`](R) reader structure"]
impl crate::Readable for C7crSpec {}
#[doc = "`write(|w| ..)` method takes [`c7cr::W`](W) writer structure"]
impl crate::Writable for C7crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C7CR to value 0"]
impl crate::Resettable for C7crSpec {}
