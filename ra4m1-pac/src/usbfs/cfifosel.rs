#[doc = "Register `CFIFOSEL` reader"]
pub type R = crate::R<CFIFOSEL_SPEC>;
#[doc = "Register `CFIFOSEL` writer"]
pub type W = crate::W<CFIFOSEL_SPEC>;
#[doc = "Field `CURPIPE` reader - CFIFO Port Access Pipe Specification"]
pub type CURPIPE_R = crate::FieldReader<CURPIPE_A>;
#[doc = "CFIFO Port Access Pipe Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURPIPE_A {
    #[doc = "0: DCP (Default control pipe)"]
    _0000 = 0,
    #[doc = "1: Pipe 1"]
    _0001 = 1,
    #[doc = "2: Pipe 2"]
    _0010 = 2,
    #[doc = "3: Pipe 3"]
    _0011 = 3,
    #[doc = "4: Pipe 4"]
    _0100 = 4,
    #[doc = "5: Pipe 5"]
    _0101 = 5,
    #[doc = "6: Pipe 6"]
    _0110 = 6,
    #[doc = "7: Pipe 7"]
    _0111 = 7,
    #[doc = "8: Pipe 8"]
    _1000 = 8,
    #[doc = "9: Pipe 9"]
    _1001 = 9,
}
impl From<CURPIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CURPIPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CURPIPE_A {
    type Ux = u8;
}
impl CURPIPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CURPIPE_A> {
        match self.bits {
            0 => Some(CURPIPE_A::_0000),
            1 => Some(CURPIPE_A::_0001),
            2 => Some(CURPIPE_A::_0010),
            3 => Some(CURPIPE_A::_0011),
            4 => Some(CURPIPE_A::_0100),
            5 => Some(CURPIPE_A::_0101),
            6 => Some(CURPIPE_A::_0110),
            7 => Some(CURPIPE_A::_0111),
            8 => Some(CURPIPE_A::_1000),
            9 => Some(CURPIPE_A::_1001),
            _ => None,
        }
    }
    #[doc = "DCP (Default control pipe)"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CURPIPE_A::_0000
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CURPIPE_A::_0001
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CURPIPE_A::_0010
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CURPIPE_A::_0011
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CURPIPE_A::_0100
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CURPIPE_A::_0101
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CURPIPE_A::_0110
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CURPIPE_A::_0111
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CURPIPE_A::_1000
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CURPIPE_A::_1001
    }
}
#[doc = "Field `CURPIPE` writer - CFIFO Port Access Pipe Specification"]
pub type CURPIPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CURPIPE_A>;
impl<'a, REG, const O: u8> CURPIPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCP (Default control pipe)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0000)
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0001)
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0010)
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0011)
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0100)
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0101)
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0110)
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0111)
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_1000)
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_1001)
    }
}
#[doc = "Field `ISEL` reader - CFIFO Port Access Direction When DCP is Selected"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "CFIFO Port Access Direction When DCP is Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: Reading from the buffer memory is selected"]
    _0 = 0,
    #[doc = "1: Writing to the buffer memory is selected"]
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    #[doc = "Reading from the buffer memory is selected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    #[doc = "Writing to the buffer memory is selected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
#[doc = "Field `ISEL` writer - CFIFO Port Access Direction When DCP is Selected"]
pub type ISEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ISEL_A>;
impl<'a, REG, const O: u8> ISEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reading from the buffer memory is selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_0)
    }
    #[doc = "Writing to the buffer memory is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_1)
    }
}
#[doc = "Field `BIGEND` reader - CFIFO Port Endian Control"]
pub type BIGEND_R = crate::BitReader<BIGEND_A>;
#[doc = "CFIFO Port Endian Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIGEND_A {
    #[doc = "0: Little endian"]
    _0 = 0,
    #[doc = "1: Big endian"]
    _1 = 1,
}
impl From<BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
impl BIGEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIGEND_A {
        match self.bits {
            false => BIGEND_A::_0,
            true => BIGEND_A::_1,
        }
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIGEND_A::_0
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIGEND_A::_1
    }
}
#[doc = "Field `BIGEND` writer - CFIFO Port Endian Control"]
pub type BIGEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BIGEND_A>;
impl<'a, REG, const O: u8> BIGEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BIGEND_A::_0)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BIGEND_A::_1)
    }
}
#[doc = "Field `MBW` reader - CFIFO Port Access Bit Width"]
pub type MBW_R = crate::BitReader<MBW_A>;
#[doc = "CFIFO Port Access Bit Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBW_A {
    #[doc = "0: 8-bit width"]
    _0 = 0,
    #[doc = "1: 16-bit width"]
    _1 = 1,
}
impl From<MBW_A> for bool {
    #[inline(always)]
    fn from(variant: MBW_A) -> Self {
        variant as u8 != 0
    }
}
impl MBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBW_A {
        match self.bits {
            false => MBW_A::_0,
            true => MBW_A::_1,
        }
    }
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MBW_A::_0
    }
    #[doc = "16-bit width"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MBW_A::_1
    }
}
#[doc = "Field `MBW` writer - CFIFO Port Access Bit Width"]
pub type MBW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MBW_A>;
impl<'a, REG, const O: u8> MBW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MBW_A::_0)
    }
    #[doc = "16-bit width"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MBW_A::_1)
    }
}
#[doc = "Buffer Pointer Rewind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REW_AW {
    #[doc = "0: The buffer pointer is not rewound."]
    _0 = 0,
    #[doc = "1: The buffer pointer is rewound."]
    _1 = 1,
}
impl From<REW_AW> for bool {
    #[inline(always)]
    fn from(variant: REW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REW` writer - Buffer Pointer Rewind"]
pub type REW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, REW_AW>;
impl<'a, REG, const O: u8> REW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The buffer pointer is not rewound."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REW_AW::_0)
    }
    #[doc = "The buffer pointer is rewound."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REW_AW::_1)
    }
}
#[doc = "Field `RCNT` reader - Read Count Mode"]
pub type RCNT_R = crate::BitReader<RCNT_A>;
#[doc = "Read Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCNT_A {
    #[doc = "0: The DTLN\\[8:0\\]
bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\]
bit value is cleared when all the data has been read from only a single plane.)"]
    _0 = 0,
    #[doc = "1: The DTLN\\[8:0\\]
bits are decremented each time the receive data is read from the CFIFO."]
    _1 = 1,
}
impl From<RCNT_A> for bool {
    #[inline(always)]
    fn from(variant: RCNT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCNT_A {
        match self.bits {
            false => RCNT_A::_0,
            true => RCNT_A::_1,
        }
    }
    #[doc = "The DTLN\\[8:0\\]
bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\]
bit value is cleared when all the data has been read from only a single plane.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCNT_A::_0
    }
    #[doc = "The DTLN\\[8:0\\]
bits are decremented each time the receive data is read from the CFIFO."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCNT_A::_1
    }
}
#[doc = "Field `RCNT` writer - Read Count Mode"]
pub type RCNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RCNT_A>;
impl<'a, REG, const O: u8> RCNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DTLN\\[8:0\\]
bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\]
bit value is cleared when all the data has been read from only a single plane.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCNT_A::_0)
    }
    #[doc = "The DTLN\\[8:0\\]
bits are decremented each time the receive data is read from the CFIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCNT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(&self) -> CURPIPE_R {
        CURPIPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - CFIFO Port Access Direction When DCP is Selected"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CFIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(&self) -> BIGEND_R {
        BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&self) -> MBW_R {
        MBW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    #[must_use]
    pub fn curpipe(&mut self) -> CURPIPE_W<CFIFOSEL_SPEC, 0> {
        CURPIPE_W::new(self)
    }
    #[doc = "Bit 5 - CFIFO Port Access Direction When DCP is Selected"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<CFIFOSEL_SPEC, 5> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 8 - CFIFO Port Endian Control"]
    #[inline(always)]
    #[must_use]
    pub fn bigend(&mut self) -> BIGEND_W<CFIFOSEL_SPEC, 8> {
        BIGEND_W::new(self)
    }
    #[doc = "Bit 10 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    #[must_use]
    pub fn mbw(&mut self) -> MBW_W<CFIFOSEL_SPEC, 10> {
        MBW_W::new(self)
    }
    #[doc = "Bit 14 - Buffer Pointer Rewind"]
    #[inline(always)]
    #[must_use]
    pub fn rew(&mut self) -> REW_W<CFIFOSEL_SPEC, 14> {
        REW_W::new(self)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rcnt(&mut self) -> RCNT_W<CFIFOSEL_SPEC, 15> {
        RCNT_W::new(self)
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
#[doc = "CFIFO Port Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfifosel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfifosel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFIFOSEL_SPEC;
impl crate::RegisterSpec for CFIFOSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfifosel::R`](R) reader structure"]
impl crate::Readable for CFIFOSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfifosel::W`](W) writer structure"]
impl crate::Writable for CFIFOSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFIFOSEL to value 0"]
impl crate::Resettable for CFIFOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
