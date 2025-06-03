#[doc = "Register `AR` reader"]
pub type R = crate::R<ArSpec>;
#[doc = "Register `AR` writer"]
pub type W = crate::W<ArSpec>;
#[doc = "Field `ADDRESS` reader - 31 0\\]: Address Address to be send to the external Flash memory Writes to this field are ignored when BUSY = 0 or when FMODE = 11 (memory-mapped mode). In dual flash mode, ADDRESS\\[0\\] is automatically stuck to 0 as the address should always be even"]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - 31 0\\]: Address Address to be send to the external Flash memory Writes to this field are ignored when BUSY = 0 or when FMODE = 11 (memory-mapped mode). In dual flash mode, ADDRESS\\[0\\] is automatically stuck to 0 as the address should always be even"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31 0\\]: Address Address to be send to the external Flash memory Writes to this field are ignored when BUSY = 0 or when FMODE = 11 (memory-mapped mode). In dual flash mode, ADDRESS\\[0\\] is automatically stuck to 0 as the address should always be even"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31 0\\]: Address Address to be send to the external Flash memory Writes to this field are ignored when BUSY = 0 or when FMODE = 11 (memory-mapped mode). In dual flash mode, ADDRESS\\[0\\] is automatically stuck to 0 as the address should always be even"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<ArSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "QUADSPI address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArSpec;
impl crate::RegisterSpec for ArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar::R`](R) reader structure"]
impl crate::Readable for ArSpec {}
#[doc = "`write(|w| ..)` method takes [`ar::W`](W) writer structure"]
impl crate::Writable for ArSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for ArSpec {}
