#[doc = "Register `RWKAR` reader"]
pub type R = crate::R<RWKAR_SPEC>;
#[doc = "Register `RWKAR` writer"]
pub type W = crate::W<RWKAR_SPEC>;
#[doc = "Field `DAYW` reader - Day-of-Week Counting"]
pub type DAYW_R = crate::FieldReader<DAYW_A>;
#[doc = "Day-of-Week Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAYW_A {
    #[doc = "0: Sunday"]
    _000 = 0,
    #[doc = "1: Monday"]
    _001 = 1,
    #[doc = "2: Tuesday"]
    _010 = 2,
    #[doc = "3: Wednesday"]
    _011 = 3,
    #[doc = "4: Thursday"]
    _100 = 4,
    #[doc = "5: Friday"]
    _101 = 5,
    #[doc = "6: Saturday"]
    _110 = 6,
    #[doc = "7: Setting Prohibited"]
    _111 = 7,
}
impl From<DAYW_A> for u8 {
    #[inline(always)]
    fn from(variant: DAYW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAYW_A {
    type Ux = u8;
}
impl DAYW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAYW_A {
        match self.bits {
            0 => DAYW_A::_000,
            1 => DAYW_A::_001,
            2 => DAYW_A::_010,
            3 => DAYW_A::_011,
            4 => DAYW_A::_100,
            5 => DAYW_A::_101,
            6 => DAYW_A::_110,
            7 => DAYW_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DAYW_A::_000
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DAYW_A::_001
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DAYW_A::_010
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DAYW_A::_011
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DAYW_A::_100
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DAYW_A::_101
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DAYW_A::_110
    }
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DAYW_A::_111
    }
}
#[doc = "Field `DAYW` writer - Day-of-Week Counting"]
pub type DAYW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, DAYW_A>;
impl<'a, REG, const O: u8> DAYW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_000)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_001)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_010)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_011)
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_100)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_101)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_110)
    }
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_111)
    }
}
#[doc = "Field `ENB` reader - Compare enable"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: The register value is not compared with the RWKCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RWKCNT counter value."]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "The register value is not compared with the RWKCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "The register value is compared with the RWKCNT counter value."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - Compare enable"]
pub type ENB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENB_A>;
impl<'a, REG, const O: u8> ENB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The register value is not compared with the RWKCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    #[doc = "The register value is compared with the RWKCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Day-of-Week Counting"]
    #[inline(always)]
    pub fn dayw(&self) -> DAYW_R {
        DAYW_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day-of-Week Counting"]
    #[inline(always)]
    #[must_use]
    pub fn dayw(&mut self) -> DAYW_W<RWKAR_SPEC, 0> {
        DAYW_W::new(self)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<RWKAR_SPEC, 7> {
        ENB_W::new(self)
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
#[doc = "Day-of-Week Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwkar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwkar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWKAR_SPEC;
impl crate::RegisterSpec for RWKAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rwkar::R`](R) reader structure"]
impl crate::Readable for RWKAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rwkar::W`](W) writer structure"]
impl crate::Writable for RWKAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWKAR to value 0"]
impl crate::Resettable for RWKAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
