#[doc = "Register `MDIOS_RDFR` reader"]
pub type R = crate::R<MdiosRdfrSpec>;
#[doc = "Field `RDF` reader - Read flags for MDIO registers 0 to 31"]
pub type RdfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn rdf(&self) -> RdfR {
        RdfR::new(self.bits)
    }
}
#[doc = "MDIOS read flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_rdfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosRdfrSpec;
impl crate::RegisterSpec for MdiosRdfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_rdfr::R`](R) reader structure"]
impl crate::Readable for MdiosRdfrSpec {}
#[doc = "`reset()` method sets MDIOS_RDFR to value 0"]
impl crate::Resettable for MdiosRdfrSpec {}
