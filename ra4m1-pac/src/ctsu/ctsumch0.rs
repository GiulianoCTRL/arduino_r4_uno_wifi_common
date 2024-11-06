#[doc = "Register `CTSUMCH0` reader"]
pub type R = crate::R<CTSUMCH0_SPEC>;
#[doc = "Register `CTSUMCH0` writer"]
pub type W = crate::W<CTSUMCH0_SPEC>;
#[doc = "Field `CTSUMCH0` reader - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\]
bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
pub type CTSUMCH0_R = crate::FieldReader<CTSUMCH0_A>;
#[doc = "CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\]
bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUMCH0_A(u8);
impl From<CTSUMCH0_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUMCH0_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUMCH0_A {
    type Ux = u8;
}
#[doc = "Field `CTSUMCH0` writer - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\]
bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
pub type CTSUMCH0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CTSUMCH0_A>;
impl R {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\]
bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
    #[inline(always)]
    pub fn ctsumch0(&self) -> CTSUMCH0_R {
        CTSUMCH0_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 0. Note1: Writing to these bits is only enabled in self-capacitance single-scan mode (CTSUCR1.CTSUMD\\[1:0\\]
bits = 00b). Note2: If the value of CTSUMCH0 was set to b'111111 in mode other than self-capacitor single scan mode, the measurement is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn ctsumch0(&mut self) -> CTSUMCH0_W<CTSUMCH0_SPEC, 0> {
        CTSUMCH0_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CTSU Measurement Channel Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsumch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsumch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUMCH0_SPEC;
impl crate::RegisterSpec for CTSUMCH0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsumch0::R`](R) reader structure"]
impl crate::Readable for CTSUMCH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsumch0::W`](W) writer structure"]
impl crate::Writable for CTSUMCH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUMCH0 to value 0x3f"]
impl crate::Resettable for CTSUMCH0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
