#[doc = "Register `AGTMR2` reader"]
pub type R = crate::R<AGTMR2_SPEC>;
#[doc = "Register `AGTMR2` writer"]
pub type W = crate::W<AGTMR2_SPEC>;
#[doc = "Field `CKS` reader - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
pub type CKS_R = crate::FieldReader<CKS_A>;
#[doc = "AGTLCLK/AGTSCLK count source clock frequency division ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: 1/1"]
    _000 = 0,
    #[doc = "1: 1/2"]
    _001 = 1,
    #[doc = "2: 1/4"]
    _010 = 2,
    #[doc = "3: 1/8"]
    _011 = 3,
    #[doc = "4: 1/16"]
    _100 = 4,
    #[doc = "5: 1/32"]
    _101 = 5,
    #[doc = "6: 1/64"]
    _110 = 6,
    #[doc = "7: 1/128."]
    _111 = 7,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKS_A {
    type Ux = u8;
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_000,
            1 => CKS_A::_001,
            2 => CKS_A::_010,
            3 => CKS_A::_011,
            4 => CKS_A::_100,
            5 => CKS_A::_101,
            6 => CKS_A::_110,
            7 => CKS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKS_A::_000
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKS_A::_001
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKS_A::_010
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKS_A::_011
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKS_A::_100
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKS_A::_101
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CKS_A::_110
    }
    #[doc = "1/128."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CKS_A::_111
    }
}
#[doc = "Field `CKS` writer - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
pub type CKS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CKS_A>;
impl<'a, REG, const O: u8> CKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_000)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_001)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_010)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_011)
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_100)
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_101)
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_110)
    }
    #[doc = "1/128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_111)
    }
}
#[doc = "Field `LPM` reader - Low Power Mode"]
pub type LPM_R = crate::BitReader<LPM_A>;
#[doc = "Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Low Power mode"]
    _1 = 1,
}
impl From<LPM_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPM_A {
        match self.bits {
            false => LPM_A::_0,
            true => LPM_A::_1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPM_A::_0
    }
    #[doc = "Low Power mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPM_A::_1
    }
}
#[doc = "Field `LPM` writer - Low Power Mode"]
pub type LPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LPM_A>;
impl<'a, REG, const O: u8> LPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LPM_A::_0)
    }
    #[doc = "Low Power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LPM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<AGTMR2_SPEC, 0> {
        CKS_W::new(self)
    }
    #[doc = "Bit 7 - Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<AGTMR2_SPEC, 7> {
        LPM_W::new(self)
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
#[doc = "AGT Mode Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGTMR2_SPEC;
impl crate::RegisterSpec for AGTMR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtmr2::R`](R) reader structure"]
impl crate::Readable for AGTMR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agtmr2::W`](W) writer structure"]
impl crate::Writable for AGTMR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTMR2 to value 0"]
impl crate::Resettable for AGTMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
