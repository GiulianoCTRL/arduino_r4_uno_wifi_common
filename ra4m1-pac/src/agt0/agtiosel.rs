#[doc = "Register `AGTIOSEL` reader"]
pub type R = crate::R<AGTIOSEL_SPEC>;
#[doc = "Register `AGTIOSEL` writer"]
pub type W = crate::W<AGTIOSEL_SPEC>;
#[doc = "Field `SEL` reader - AGTIO pin select"]
pub type SEL_R = crate::FieldReader<SEL_A>;
#[doc = "AGTIO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Select the AGTIOn except for below pins"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    _10 = 2,
    #[doc = "3: Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    _11 = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEL_A {
    type Ux = u8;
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::_00,
            1 => SEL_A::_01,
            2 => SEL_A::_10,
            3 => SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Select the AGTIOn except for below pins"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEL_A::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEL_A::_01
    }
    #[doc = "Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL_A::_10
    }
    #[doc = "Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL_A::_11
    }
}
#[doc = "Field `SEL` writer - AGTIO pin select"]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SEL_A>;
impl<'a, REG, const O: u8> SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the AGTIOn except for below pins"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_01)
    }
    #[doc = "Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_10)
    }
    #[doc = "Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_11)
    }
}
#[doc = "Field `TIES` reader - AGTIO input enable"]
pub type TIES_R = crate::BitReader<TIES_A>;
#[doc = "AGTIO input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIES_A {
    #[doc = "0: External event input is disabled during Software Standby mode"]
    _0 = 0,
    #[doc = "1: External event input is enabled during Software Standby mode."]
    _1 = 1,
}
impl From<TIES_A> for bool {
    #[inline(always)]
    fn from(variant: TIES_A) -> Self {
        variant as u8 != 0
    }
}
impl TIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIES_A {
        match self.bits {
            false => TIES_A::_0,
            true => TIES_A::_1,
        }
    }
    #[doc = "External event input is disabled during Software Standby mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIES_A::_0
    }
    #[doc = "External event input is enabled during Software Standby mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIES_A::_1
    }
}
#[doc = "Field `TIES` writer - AGTIO input enable"]
pub type TIES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIES_A>;
impl<'a, REG, const O: u8> TIES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External event input is disabled during Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIES_A::_0)
    }
    #[doc = "External event input is enabled during Software Standby mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIES_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - AGTIO pin select"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - AGTIO input enable"]
    #[inline(always)]
    pub fn ties(&self) -> TIES_R {
        TIES_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AGTIO pin select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<AGTIOSEL_SPEC, 0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 4 - AGTIO input enable"]
    #[inline(always)]
    #[must_use]
    pub fn ties(&mut self) -> TIES_W<AGTIOSEL_SPEC, 4> {
        TIES_W::new(self)
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
#[doc = "AGT Pin Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtiosel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtiosel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGTIOSEL_SPEC;
impl crate::RegisterSpec for AGTIOSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtiosel::R`](R) reader structure"]
impl crate::Readable for AGTIOSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agtiosel::W`](W) writer structure"]
impl crate::Writable for AGTIOSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTIOSEL to value 0"]
impl crate::Resettable for AGTIOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
