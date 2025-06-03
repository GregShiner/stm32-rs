#[doc = "Register `RESP4R` reader"]
pub type R = crate::R<Resp4rSpec>;
#[doc = "Field `CARDSTATUS4` reader - see Table404."]
pub type Cardstatus4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus4(&self) -> Cardstatus4R {
        Cardstatus4R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp4rSpec;
impl crate::RegisterSpec for Resp4rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4r::R`](R) reader structure"]
impl crate::Readable for Resp4rSpec {}
#[doc = "`reset()` method sets RESP4R to value 0"]
impl crate::Resettable for Resp4rSpec {}
