#[doc = "Register `ECCR` reader"]
pub type R = crate::R<EccrSpec>;
#[doc = "Field `ECC` reader - ECC result This field contains the value computed by the ECC computation logic. Table167 describes the contents of these bit fields."]
pub type EccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result This field contains the value computed by the ECC computation logic. Table167 describes the contents of these bit fields."]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(self.bits)
    }
}
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccrSpec;
impl crate::RegisterSpec for EccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for EccrSpec {}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for EccrSpec {}
