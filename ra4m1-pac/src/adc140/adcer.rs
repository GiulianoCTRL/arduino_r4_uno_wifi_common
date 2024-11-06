#[doc = "Register `ADCER` reader"]
pub type R = crate::R<ADCER_SPEC>;
#[doc = "Register `ADCER` writer"]
pub type W = crate::W<ADCER_SPEC>;
#[doc = "Field `ADPRC` reader - A/D Conversion Accuracy Specify"]
pub type ADPRC_R = crate::FieldReader<ADPRC_A>;
#[doc = "A/D Conversion Accuracy Specify\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADPRC_A {
    #[doc = "0: A/D conversion is performed with 12-bit accuracy."]
    _00 = 0,
    #[doc = "3: A/D conversion is performed with 14-bit accuracy."]
    _11 = 3,
}
impl From<ADPRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADPRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADPRC_A {
    type Ux = u8;
}
impl ADPRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADPRC_A> {
        match self.bits {
            0 => Some(ADPRC_A::_00),
            3 => Some(ADPRC_A::_11),
            _ => None,
        }
    }
    #[doc = "A/D conversion is performed with 12-bit accuracy."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADPRC_A::_00
    }
    #[doc = "A/D conversion is performed with 14-bit accuracy."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADPRC_A::_11
    }
}
#[doc = "Field `ADPRC` writer - A/D Conversion Accuracy Specify"]
pub type ADPRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADPRC_A>;
impl<'a, REG, const O: u8> ADPRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A/D conversion is performed with 12-bit accuracy."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ADPRC_A::_00)
    }
    #[doc = "A/D conversion is performed with 14-bit accuracy."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ADPRC_A::_11)
    }
}
#[doc = "Field `ACE` reader - A/D Data Register Automatic Clearing Enable"]
pub type ACE_R = crate::BitReader<ACE_A>;
#[doc = "A/D Data Register Automatic Clearing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACE_A {
    #[doc = "0: Disables automatic clearing."]
    _0 = 0,
    #[doc = "1: Enables automatic clearing."]
    _1 = 1,
}
impl From<ACE_A> for bool {
    #[inline(always)]
    fn from(variant: ACE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACE_A {
        match self.bits {
            false => ACE_A::_0,
            true => ACE_A::_1,
        }
    }
    #[doc = "Disables automatic clearing."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACE_A::_0
    }
    #[doc = "Enables automatic clearing."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACE_A::_1
    }
}
#[doc = "Field `ACE` writer - A/D Data Register Automatic Clearing Enable"]
pub type ACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACE_A>;
impl<'a, REG, const O: u8> ACE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables automatic clearing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACE_A::_0)
    }
    #[doc = "Enables automatic clearing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACE_A::_1)
    }
}
#[doc = "Field `DIAGVAL` reader - Self-Diagnosis Conversion Voltage Select"]
pub type DIAGVAL_R = crate::FieldReader<DIAGVAL_A>;
#[doc = "Self-Diagnosis Conversion Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGVAL_A {
    #[doc = "0: When the self-diagnosis fixation mode is selected, it set prohibits it."]
    _00 = 0,
    #[doc = "1: The self-diagnosis by using the voltage of 0V."]
    _01 = 1,
    #[doc = "2: The self-diagnosis by using the voltage of reference supply x 1/2."]
    _10 = 2,
    #[doc = "3: The self-diagnosis by using the voltage of the reference supply."]
    _11 = 3,
}
impl From<DIAGVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIAGVAL_A {
    type Ux = u8;
}
impl DIAGVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIAGVAL_A {
        match self.bits {
            0 => DIAGVAL_A::_00,
            1 => DIAGVAL_A::_01,
            2 => DIAGVAL_A::_10,
            3 => DIAGVAL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGVAL_A::_00
    }
    #[doc = "The self-diagnosis by using the voltage of 0V."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGVAL_A::_01
    }
    #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGVAL_A::_10
    }
    #[doc = "The self-diagnosis by using the voltage of the reference supply."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGVAL_A::_11
    }
}
#[doc = "Field `DIAGVAL` writer - Self-Diagnosis Conversion Voltage Select"]
pub type DIAGVAL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DIAGVAL_A>;
impl<'a, REG, const O: u8> DIAGVAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_00)
    }
    #[doc = "The self-diagnosis by using the voltage of 0V."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_01)
    }
    #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_10)
    }
    #[doc = "The self-diagnosis by using the voltage of the reference supply."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_11)
    }
}
#[doc = "Field `DIAGLD` reader - Self-Diagnosis Mode Select"]
pub type DIAGLD_R = crate::BitReader<DIAGLD_A>;
#[doc = "Self-Diagnosis Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIAGLD_A {
    #[doc = "0: Rotation mode for self-diagnosis voltage"]
    _0 = 0,
    #[doc = "1: Fixed mode for self-diagnosis voltage"]
    _1 = 1,
}
impl From<DIAGLD_A> for bool {
    #[inline(always)]
    fn from(variant: DIAGLD_A) -> Self {
        variant as u8 != 0
    }
}
impl DIAGLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIAGLD_A {
        match self.bits {
            false => DIAGLD_A::_0,
            true => DIAGLD_A::_1,
        }
    }
    #[doc = "Rotation mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIAGLD_A::_0
    }
    #[doc = "Fixed mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIAGLD_A::_1
    }
}
#[doc = "Field `DIAGLD` writer - Self-Diagnosis Mode Select"]
pub type DIAGLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DIAGLD_A>;
impl<'a, REG, const O: u8> DIAGLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rotation mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGLD_A::_0)
    }
    #[doc = "Fixed mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGLD_A::_1)
    }
}
#[doc = "Field `DIAGM` reader - Self-Diagnosis Enable"]
pub type DIAGM_R = crate::BitReader<DIAGM_A>;
#[doc = "Self-Diagnosis Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIAGM_A {
    #[doc = "0: Disables self-diagnosis of A/D converter."]
    _0 = 0,
    #[doc = "1: Enables self-diagnosis of A/D converter."]
    _1 = 1,
}
impl From<DIAGM_A> for bool {
    #[inline(always)]
    fn from(variant: DIAGM_A) -> Self {
        variant as u8 != 0
    }
}
impl DIAGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIAGM_A {
        match self.bits {
            false => DIAGM_A::_0,
            true => DIAGM_A::_1,
        }
    }
    #[doc = "Disables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIAGM_A::_0
    }
    #[doc = "Enables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIAGM_A::_1
    }
}
#[doc = "Field `DIAGM` writer - Self-Diagnosis Enable"]
pub type DIAGM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DIAGM_A>;
impl<'a, REG, const O: u8> DIAGM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGM_A::_0)
    }
    #[doc = "Enables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGM_A::_1)
    }
}
#[doc = "Field `ADRFMT` reader - A/D Data Register Format Select"]
pub type ADRFMT_R = crate::BitReader<ADRFMT_A>;
#[doc = "A/D Data Register Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRFMT_A {
    #[doc = "0: Flush-right is selected for the A/D data register format."]
    _0 = 0,
    #[doc = "1: Flush-left is selected for the A/D data register format."]
    _1 = 1,
}
impl From<ADRFMT_A> for bool {
    #[inline(always)]
    fn from(variant: ADRFMT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRFMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRFMT_A {
        match self.bits {
            false => ADRFMT_A::_0,
            true => ADRFMT_A::_1,
        }
    }
    #[doc = "Flush-right is selected for the A/D data register format."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADRFMT_A::_0
    }
    #[doc = "Flush-left is selected for the A/D data register format."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADRFMT_A::_1
    }
}
#[doc = "Field `ADRFMT` writer - A/D Data Register Format Select"]
pub type ADRFMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADRFMT_A>;
impl<'a, REG, const O: u8> ADRFMT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flush-right is selected for the A/D data register format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRFMT_A::_0)
    }
    #[doc = "Flush-left is selected for the A/D data register format."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRFMT_A::_1)
    }
}
impl R {
    #[doc = "Bits 1:2 - A/D Conversion Accuracy Specify"]
    #[inline(always)]
    pub fn adprc(&self) -> ADPRC_R {
        ADPRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 5 - A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    pub fn ace(&self) -> ACE_R {
        ACE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    pub fn diagval(&self) -> DIAGVAL_R {
        DIAGVAL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Self-Diagnosis Mode Select"]
    #[inline(always)]
    pub fn diagld(&self) -> DIAGLD_R {
        DIAGLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self-Diagnosis Enable"]
    #[inline(always)]
    pub fn diagm(&self) -> DIAGM_R {
        DIAGM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - A/D Data Register Format Select"]
    #[inline(always)]
    pub fn adrfmt(&self) -> ADRFMT_R {
        ADRFMT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - A/D Conversion Accuracy Specify"]
    #[inline(always)]
    #[must_use]
    pub fn adprc(&mut self) -> ADPRC_W<ADCER_SPEC, 1> {
        ADPRC_W::new(self)
    }
    #[doc = "Bit 5 - A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ace(&mut self) -> ACE_W<ADCER_SPEC, 5> {
        ACE_W::new(self)
    }
    #[doc = "Bits 8:9 - Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn diagval(&mut self) -> DIAGVAL_W<ADCER_SPEC, 8> {
        DIAGVAL_W::new(self)
    }
    #[doc = "Bit 10 - Self-Diagnosis Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn diagld(&mut self) -> DIAGLD_W<ADCER_SPEC, 10> {
        DIAGLD_W::new(self)
    }
    #[doc = "Bit 11 - Self-Diagnosis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn diagm(&mut self) -> DIAGM_W<ADCER_SPEC, 11> {
        DIAGM_W::new(self)
    }
    #[doc = "Bit 15 - A/D Data Register Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn adrfmt(&mut self) -> ADRFMT_W<ADCER_SPEC, 15> {
        ADRFMT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Control Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCER_SPEC;
impl crate::RegisterSpec for ADCER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcer::R`](R) reader structure"]
impl crate::Readable for ADCER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcer::W`](W) writer structure"]
impl crate::Writable for ADCER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCER to value 0"]
impl crate::Resettable for ADCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
