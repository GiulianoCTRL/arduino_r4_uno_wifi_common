#[doc = "Register `ECSR` reader"]
pub type R = crate::R<ECSR_SPEC>;
#[doc = "Register `ECSR` writer"]
pub type W = crate::W<ECSR_SPEC>;
#[doc = "Field `SEF` reader - Stuff Error Flag"]
pub type SEF_R = crate::BitReader<SEF_A>;
#[doc = "Stuff Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEF_A {
    #[doc = "0: No stuff error detected"]
    _0 = 0,
    #[doc = "1: Stuff error detected"]
    _1 = 1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as u8 != 0
    }
}
impl SEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::_0,
            true => SEF_A::_1,
        }
    }
    #[doc = "No stuff error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEF_A::_0
    }
    #[doc = "Stuff error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEF_A::_1
    }
}
#[doc = "Field `SEF` writer - Stuff Error Flag"]
pub type SEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEF_A>;
impl<'a, REG, const O: u8> SEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No stuff error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEF_A::_0)
    }
    #[doc = "Stuff error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEF_A::_1)
    }
}
#[doc = "Field `FEF` reader - Form Error Flag"]
pub type FEF_R = crate::BitReader<FEF_A>;
#[doc = "Form Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    #[doc = "0: No form error detected"]
    _0 = 0,
    #[doc = "1: Form error detected"]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "No form error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEF_A::_0
    }
    #[doc = "Form error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEF_A::_1
    }
}
#[doc = "Field `FEF` writer - Form Error Flag"]
pub type FEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FEF_A>;
impl<'a, REG, const O: u8> FEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No form error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::_0)
    }
    #[doc = "Form error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::_1)
    }
}
#[doc = "Field `AEF` reader - ACK Error Flag"]
pub type AEF_R = crate::BitReader<AEF_A>;
#[doc = "ACK Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEF_A {
    #[doc = "0: No ACK error detected"]
    _0 = 0,
    #[doc = "1: ACK error detected"]
    _1 = 1,
}
impl From<AEF_A> for bool {
    #[inline(always)]
    fn from(variant: AEF_A) -> Self {
        variant as u8 != 0
    }
}
impl AEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AEF_A {
        match self.bits {
            false => AEF_A::_0,
            true => AEF_A::_1,
        }
    }
    #[doc = "No ACK error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEF_A::_0
    }
    #[doc = "ACK error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEF_A::_1
    }
}
#[doc = "Field `AEF` writer - ACK Error Flag"]
pub type AEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AEF_A>;
impl<'a, REG, const O: u8> AEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AEF_A::_0)
    }
    #[doc = "ACK error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AEF_A::_1)
    }
}
#[doc = "Field `CEF` reader - CRC Error Flag"]
pub type CEF_R = crate::BitReader<CEF_A>;
#[doc = "CRC Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEF_A {
    #[doc = "0: No CRC error detected"]
    _0 = 0,
    #[doc = "1: CRC error detected"]
    _1 = 1,
}
impl From<CEF_A> for bool {
    #[inline(always)]
    fn from(variant: CEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEF_A {
        match self.bits {
            false => CEF_A::_0,
            true => CEF_A::_1,
        }
    }
    #[doc = "No CRC error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEF_A::_0
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEF_A::_1
    }
}
#[doc = "Field `CEF` writer - CRC Error Flag"]
pub type CEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CEF_A>;
impl<'a, REG, const O: u8> CEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CRC error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CEF_A::_0)
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CEF_A::_1)
    }
}
#[doc = "Field `BE1F` reader - Bit Error (recessive) Flag"]
pub type BE1F_R = crate::BitReader<BE1F_A>;
#[doc = "Bit Error (recessive) Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BE1F_A {
    #[doc = "0: No bit error (recessive) detected"]
    _0 = 0,
    #[doc = "1: Bit error (recessive) detected"]
    _1 = 1,
}
impl From<BE1F_A> for bool {
    #[inline(always)]
    fn from(variant: BE1F_A) -> Self {
        variant as u8 != 0
    }
}
impl BE1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BE1F_A {
        match self.bits {
            false => BE1F_A::_0,
            true => BE1F_A::_1,
        }
    }
    #[doc = "No bit error (recessive) detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BE1F_A::_0
    }
    #[doc = "Bit error (recessive) detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BE1F_A::_1
    }
}
#[doc = "Field `BE1F` writer - Bit Error (recessive) Flag"]
pub type BE1F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BE1F_A>;
impl<'a, REG, const O: u8> BE1F_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bit error (recessive) detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BE1F_A::_0)
    }
    #[doc = "Bit error (recessive) detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BE1F_A::_1)
    }
}
#[doc = "Field `BE0F` reader - Bit Error (dominant) Flag"]
pub type BE0F_R = crate::BitReader<BE0F_A>;
#[doc = "Bit Error (dominant) Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BE0F_A {
    #[doc = "0: No bit error (dominant) detected"]
    _0 = 0,
    #[doc = "1: Bit error (dominant) detected"]
    _1 = 1,
}
impl From<BE0F_A> for bool {
    #[inline(always)]
    fn from(variant: BE0F_A) -> Self {
        variant as u8 != 0
    }
}
impl BE0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BE0F_A {
        match self.bits {
            false => BE0F_A::_0,
            true => BE0F_A::_1,
        }
    }
    #[doc = "No bit error (dominant) detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BE0F_A::_0
    }
    #[doc = "Bit error (dominant) detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BE0F_A::_1
    }
}
#[doc = "Field `BE0F` writer - Bit Error (dominant) Flag"]
pub type BE0F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BE0F_A>;
impl<'a, REG, const O: u8> BE0F_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bit error (dominant) detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BE0F_A::_0)
    }
    #[doc = "Bit error (dominant) detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BE0F_A::_1)
    }
}
#[doc = "Field `ADEF` reader - ACK Delimiter Error Flag"]
pub type ADEF_R = crate::BitReader<ADEF_A>;
#[doc = "ACK Delimiter Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEF_A {
    #[doc = "0: No ACK delimiter error detected"]
    _0 = 0,
    #[doc = "1: ACK delimiter error detected"]
    _1 = 1,
}
impl From<ADEF_A> for bool {
    #[inline(always)]
    fn from(variant: ADEF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADEF_A {
        match self.bits {
            false => ADEF_A::_0,
            true => ADEF_A::_1,
        }
    }
    #[doc = "No ACK delimiter error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEF_A::_0
    }
    #[doc = "ACK delimiter error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEF_A::_1
    }
}
#[doc = "Field `ADEF` writer - ACK Delimiter Error Flag"]
pub type ADEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADEF_A>;
impl<'a, REG, const O: u8> ADEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK delimiter error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADEF_A::_0)
    }
    #[doc = "ACK delimiter error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADEF_A::_1)
    }
}
#[doc = "Field `EDPM` reader - Error Display Mode Select"]
pub type EDPM_R = crate::BitReader<EDPM_A>;
#[doc = "Error Display Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDPM_A {
    #[doc = "0: Output of first detected error code"]
    _0 = 0,
    #[doc = "1: Output of accumulated error code"]
    _1 = 1,
}
impl From<EDPM_A> for bool {
    #[inline(always)]
    fn from(variant: EDPM_A) -> Self {
        variant as u8 != 0
    }
}
impl EDPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDPM_A {
        match self.bits {
            false => EDPM_A::_0,
            true => EDPM_A::_1,
        }
    }
    #[doc = "Output of first detected error code"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDPM_A::_0
    }
    #[doc = "Output of accumulated error code"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDPM_A::_1
    }
}
#[doc = "Field `EDPM` writer - Error Display Mode Select"]
pub type EDPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EDPM_A>;
impl<'a, REG, const O: u8> EDPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output of first detected error code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EDPM_A::_0)
    }
    #[doc = "Output of accumulated error code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EDPM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stuff Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Form Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACK Error Flag"]
    #[inline(always)]
    pub fn aef(&self) -> AEF_R {
        AEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CRC Error Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit Error (recessive) Flag"]
    #[inline(always)]
    pub fn be1f(&self) -> BE1F_R {
        BE1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit Error (dominant) Flag"]
    #[inline(always)]
    pub fn be0f(&self) -> BE0F_R {
        BE0F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACK Delimiter Error Flag"]
    #[inline(always)]
    pub fn adef(&self) -> ADEF_R {
        ADEF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Display Mode Select"]
    #[inline(always)]
    pub fn edpm(&self) -> EDPM_R {
        EDPM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stuff Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sef(&mut self) -> SEF_W<ECSR_SPEC, 0> {
        SEF_W::new(self)
    }
    #[doc = "Bit 1 - Form Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<ECSR_SPEC, 1> {
        FEF_W::new(self)
    }
    #[doc = "Bit 2 - ACK Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aef(&mut self) -> AEF_W<ECSR_SPEC, 2> {
        AEF_W::new(self)
    }
    #[doc = "Bit 3 - CRC Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cef(&mut self) -> CEF_W<ECSR_SPEC, 3> {
        CEF_W::new(self)
    }
    #[doc = "Bit 4 - Bit Error (recessive) Flag"]
    #[inline(always)]
    #[must_use]
    pub fn be1f(&mut self) -> BE1F_W<ECSR_SPEC, 4> {
        BE1F_W::new(self)
    }
    #[doc = "Bit 5 - Bit Error (dominant) Flag"]
    #[inline(always)]
    #[must_use]
    pub fn be0f(&mut self) -> BE0F_W<ECSR_SPEC, 5> {
        BE0F_W::new(self)
    }
    #[doc = "Bit 6 - ACK Delimiter Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adef(&mut self) -> ADEF_W<ECSR_SPEC, 6> {
        ADEF_W::new(self)
    }
    #[doc = "Bit 7 - Error Display Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn edpm(&mut self) -> EDPM_W<ECSR_SPEC, 7> {
        EDPM_W::new(self)
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
#[doc = "Error Code Store Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECSR_SPEC;
impl crate::RegisterSpec for ECSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecsr::R`](R) reader structure"]
impl crate::Readable for ECSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecsr::W`](W) writer structure"]
impl crate::Writable for ECSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECSR to value 0"]
impl crate::Resettable for ECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
