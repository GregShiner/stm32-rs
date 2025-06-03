#[doc = "Register `MACARPAR` reader"]
pub type R = crate::R<MacarparSpec>;
#[doc = "Register `MACARPAR` writer"]
pub type W = crate::W<MacarparSpec>;
#[doc = "Field `ARPPA` reader - ARPPA"]
pub type ArppaR = crate::FieldReader<u32>;
#[doc = "Field `ARPPA` writer - ARPPA"]
pub type ArppaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ARPPA"]
    #[inline(always)]
    pub fn arppa(&self) -> ArppaR {
        ArppaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARPPA"]
    #[inline(always)]
    pub fn arppa(&mut self) -> ArppaW<MacarparSpec> {
        ArppaW::new(self, 0)
    }
}
#[doc = "ARP address register\n\nYou can [`read`](crate::Reg::read) this register and get [`macarpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macarpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacarparSpec;
impl crate::RegisterSpec for MacarparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macarpar::R`](R) reader structure"]
impl crate::Readable for MacarparSpec {}
#[doc = "`write(|w| ..)` method takes [`macarpar::W`](W) writer structure"]
impl crate::Writable for MacarparSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACARPAR to value 0"]
impl crate::Resettable for MacarparSpec {}
