<Cabbage>
	#define IGNORE automatable(0), presetIgnore(1)
  ; #define SIZE size(3838,2082)
  #define SIZE size(1918,1028)
  form autoUpdate(1), openGL(1), pluginId("5757"), caption("borovylo"), bundle("variables.csd"), guiMode("queue"), colour(60,62,60), style("flat"), $SIZE
</Cabbage>
<CsoundSynthesizer>
<CsOptions>
-d -m0
-n
; tell Csound NOT to use its own MIDI interface, and listen for MIDI messages on all ports.
-+rtmidi=null -M0
</CsOptions>
<CsInstruments>

; sr = 44100
0dbfs = 1 ; ВНИМАНИЕ! 0dbfs всегда устанавливать 1
ksmps = 1000
nchnls = 2   ; кол-во аудио-выходов
nchnls_i = 0 ; входы

; сгенеренный файл с данными параметров и кнопками раскладок
#include "variables.csd"

; размеры и координаты
giSIZE ftgen 2, 0, 24, -7, 0

; index активной раскладки
gkLayout init 0
; index активного голоса
gkChan init 0
; index активного параметра
gkControl init 0

; HOST BPM
gkBPM init 93

; UDO УТИЛИТЫ

; вернет id параметра привязанного к СС, инкримент, минимум и максимум
opcode get_param_attrs, kkkk, k
  ; kcc - индекс CC контролера, индекс параметра в giP
  kcc xin
  ; значение инкримента параметра всегда больше 0
  k_step, k_min, k_max, k_def init 0
  vtablek kcc, giP, 0, 0, k_step, k_min, k_max, k_def
  ; вернет СС код или 0
  xout (k_step > 0 ? kcc : 0), k_step, k_min, k_max
endop

; название cabbage-канала раскладки
opcode layout_chan, S, kk
  ; kN - номер голоса
  ; kL - индекс раскладки
  kN, kL xin
  xout sprintfk("l%d%d", kN, kL)
endop

; название cabbage-канала параметра
opcode param_chan, S, kk
  ; kN - номер голоса
  ; kP - индекс параметра
  kN, kP xin
  xout sprintfk("p%d%d", kN, kP)
endop

; название cabbage-канала tab'a
opcode tab_chan, S, k
  ; kL - индекс раскладки
  kL xin
  xout sprintfk("t%d", kL)
endop

; ПОСТРОЕНИЕ GUI

; GUI-генератор tab-кнопок
instr tabs
  iW, iTH, iTW, iP init 0
  S_box init "tab_box"

  ; width рабочей области
  vtabi 10, giSIZE, iW, iTH
  ; padding
  vtabi 11, giSIZE, iP, iTH
  ; размеры tab-кнопи
  vtabi  9, giSIZE, iTW, iTH

  cabbageCreate "groupbox", sprintf( \
    {{ bounds(%d,%d,%d,%d), channel("%s"), colour(0,0,0,0), %s, $IGNORE }}, \
    iP, iP, iW, iTH, \
    S_box, \
    {{corners(0), lineThickness(0), outlineThickness(0)}} \
  )

  idx init 0
  while idx < giL_len do
    cabbageCreate "button", sprintf( \
      {{channel("%s"), bounds(%d,0,%d,%d), text("%c"), value(%d), parent("%s"), %s, %s, $IGNORE}}, \
      tab_chan(k(idx)), \
      idx * iTW, iTW, iTH, \
      65 + idx, \
      idx == 0 ? 1 : 0, \
      S_box, \
      {{colour:0(0,0,0,60), colour:1(0,0,0,120), fontColour:0(90,108,110,120), fontColour:1(255,215,215,120)}}, \
      {{outlineThickness(0), corners(0), radioGroup(1)}} \
    )
    idx += 1
  od

  turnoff
endin

; GUI-генератор layout'ов по входным данным, где
instr layouts
  ; iN (p4) - индекс инструмента
  ; iL (p5) - индекс раскладки
  iN, iL passign 4

  S_layout layout_chan iN, iL

  iW, iH init 0
  ; width рабочей области
  vtabi 10, giSIZE, iW, iH

  ; высота блока с кнопками раскладки
  ih = int(iH/3)

  cabbageCreate "groupbox", sprintf( \
    {{ bounds(%d,%d,%d,%d), channel(%s), %s }}, \
    0, iH - ih, iW, ih, \
    S_layout, \
    {{lineThickness(0), outlineThickness(0), $IGNORE}} \
  )

  cabbageCreate "image", sprintf( \
    {{parent("%s"), channel("%s-bg"), bounds(0,0,%d,%d), colour(32,38,30,255), %s, $IGNORE}}, \
    S_layout, S_layout, \
    iW, ih, \
    {{corners(0), outlineThickness(0)}} \
  )
  turnoff
endin


; GUI-генератор кнопки в groupbox раскладки
instr button
  ; iN   (p4) - номер голоса
  ; irow (p5) - индекс кнопки, giB
  ; iN, p5 passign 4

  ; атрибуты gui кнопки:
  ;  iL       - индекс раскладки
  ;  ibtn_pos - индекс giSIZE - позиция на экране
  ;  iBW, iBH - width/height кнопки
  ;  ix, iy   - координаты верхнего левого угла кнопки
  ;  iG       - отступ между кнопками и от края
  ;
  ; атрибуты параметра:
  ;  iP    - индекс параметра, giP
  ;  istep - шаг изменения по умолчанию
  ;  imin  - минимальное значение
  ;  imax  - максимальное значение
  ;  idef  - значение по умолчанию
  iP, iL, ibtn_pos, istep, imin, imax, idef, ix, iy, iBW, iBH, iG init 0

  ; модификатор размера label-шрифта параметра
  i_label_mod init 0.4
  ; модификатор размера value-шрифта параметра
  i_value_mod init 0.45

  ; размер кнопки
  vtabi 0, giSIZE, iBW, iBH
  ; padding
  vtabi 11, giSIZE, iP, iG

  ; данные кнопки - параметр, раскладка, позиция
  vtabi p5, giB, iP, iL, ibtn_pos
  ; координаты кнопки на экране
  vtabi ibtn_pos, giSIZE, ix, iy
  ; данные инициализации параметра
  vtabi iP, giP, istep, imin, imax, idef

  ; label параметра
  SL strget iP

  ; к какой раскладке относится кнопка
  S_layout layout_chan p4, iL
  ; часть названия каналов параметра
  S_param param_chan p4, iP

  ; background
  cabbageCreate "image", sprintf( \
    {{parent("%s"), channel("%s-bg"), bounds(%d,%d,%d,%d), %s, $IGNORE}}, \
    S_layout, S_param, \
    ix, iy, iBW, iBH, \
    {{corners(0), outlineThickness(0), colour(56,83,74,80), outlineColour(255,215,215,120)}} \
  )

  ; название параметра
  cabbageCreate "label", sprintf( \
    {{parent("%s"), channel("%s-label"), bounds(%d,%d,%d,%d), text("%s"), %s}}, \
    S_layout, S_param, \
    ix, iy + iG, iBW, i_label_mod * iBH, \
    strupper(SL), \
    {{corners(0), fontColour(255,215,215,120), $IGNORE}} \
  )

  ; значение параметра
  cabbageCreate "nslider", sprintf( \
    {{parent("%s"), channel("%s"), bounds(%d,%d,%d,%d), increment(%f), min(%f), max(%f), value(%f), %s, %s}}, \
    S_layout, S_param, \
    ix, iy + i_value_mod * iBH, iBW, i_value_mod * iBH, \
    istep, imin, imax, idef, \
    {{textColour(255,215,215,150), fontColour(255,255,255,160), outlineColour(0,0,0,0), colour(0,0,0,0)}}, \
    {{popupText(""), popup(0), text("")}} \
  )

  turnoff
endin


; точка входа GUI-генераторов
instr voices
  ; p4 - кол-во голосов

  ; padding
  iP, iG init 4
  ; размер рабочей области
  iW cabbageGet "form", "width"
  iH cabbageGet "form", "height"

  ; iN - индекс голоса
	iN init 0

  ; размеры кнопок в раскладке
  iBW = int((iW - 5*iP) / 4)
  iBH = int((int(iH/3) - 3*iP) / 2)
  ; рассчет координат кнопок в раскладке
  ix2 = 2*iP + iBW
  ix3 = iW - 2*(iP + iBW)
  ix4 = iW - iP - iBW
  iy2 = 2*iP + iBH
  ; размер кнопки раскладки
  vtabwi  0, giSIZE, iBW, iBH

  ; координаты кнопок
  vtabwi  5, giSIZE,  iP, iy2
  vtabwi  6, giSIZE, ix2, iy2
  vtabwi  7, giSIZE,  iP, iP
  vtabwi  8, giSIZE, ix2, iP

  vtabwi  1, giSIZE, ix3, iy2
  vtabwi  2, giSIZE, ix4, iy2
  vtabwi  3, giSIZE, ix3, iP
  vtabwi  4, giSIZE, ix4, iP

  ; размер tab-кнопки
  vtabwi  9, giSIZE, 80, 54
  ; размер рабочей области
  vtabwi 10, giSIZE, iW, iH
  ; padding
  vtabwi 11, giSIZE, iP, iG

  ; @TODO: сделать номальный канал
  cabbageCreate "nslider", sprintf( \
    {{channel("gkChan"), bounds(%d,%d,%d,%d), value(0), colour(0,0,0,0), textColour(255,215,215,160), visible(0), $IGNORE}}, \
    iW - iP - 0.5 * iBW, iP, 0.5 * iBW, 0.5 * iBH \
  )
  cabbageCreate "nslider", sprintf( \
    {{channel("gkLayout"), bounds(%d,%d,%d,%d), value(0), visible(0), $IGNORE}}, \
    iW - iP - 0.5 * iBW, iP + 0.5 * iBH + iG, 0.5 * iBW, 0.5 * iBH \
  )
  cabbageCreate "nslider", sprintf( \
    {{channel("gkParam"), bounds(%d,%d,%d,%d), value(0), visible(0), $IGNORE}}, \
    iW - iP - 0.5 * iBW, iP + 2 * (0.5 * iBH + iG),  0.5 * iBW, 0.5 * iBH \
  )

  ; генератор tab-кнопок раскладок
  scoreline_i {{ i "tabs" 0 1 }}

  iB_len = ftlen(giB) / 3

  while iN < p4 do
    ; prints " ! init voice %d", iN
    idx = giL_len

    ; генератор groupbox'ов раскладкок
    while idx > 0 do
      ; в обратном порядке, чтобы первая раскладка оказалась поверх остальных
      idx -= 1
      scoreline_i sprintf({{ i "layouts" 0 1 %d %d }}, iN, idx)
    od
    ; генератор кнопок для раскладкок
    while idx < iB_len do
      scoreline_i sprintf({{ i "button" 0 1 %d %d }}, iN, idx)
      idx += 1
    od

    iN += 1
  od

  ; k-time, не чаще раза в 0.5 секунды
  ; printks " trig next layout = %d\n", 0.5, kL_next
  turnoff
endin

instr gui_event
  ; индекс текущей раскладки обновляется через канал
  ; gkLayout chnget "gkLayout"
  ; индекс текущего голоса
  ; gkChan chnget "gkChan"

  gkLayout = chnget:k("gkLayout")
  gkParam = chnget:k("gkParam")
  gkChan = chnget:k("gkChan")

  ; onClick на tab-кнопку, переключает раскладку мышкой
  S_tabs_channels[] cabbageGetWidgetChannels "radioGroup(1)"
  kL, k_trig cabbageChanged S_tabs_channels, 0
  if k_trig == 1 && gkLayout != kL then
    event "i", "load_layout", 0, 1, gkChan, kL, gkParam
    ; @TODO: отправить Pompedulle новое значение раскладки
  endif

	gkBPM = chnget:k("HOST_BPM")
  if changed:k(gkBPM) == 1 then
    printks "HOST BPM %d\n", 0, gkBPM
  endif
endin


; загрузить новую раскладку
instr load_layout, 101
  ; p4 - номер голоса
  ; p5 - загрузить раскладку
	; p6 - текущий параметр
	if p6 != 0 then
    S_chan param_chan p4, p6
    cabbageSet sprintf("%s-bg", S_chan), "colour", 56, 83, 74, 80
    cabbageSet sprintf("%s-bg", S_chan), "outlineThickness", 0
	endif

  cabbageSet layout_chan(p4, p5), "toFront(1)"
  cabbageSetValue "gkLayout", p5
  cabbageSetValue "gkParam", 0

  ; cabageSet "gkLayout-value", sprintf({{text("%d")}}, p5)
  cabbageSet "gkParam-value", {{text("")}}
  turnoff
endin


instr set_param, 102
  ; p4 - номер голоса
	; p5 - next
	; p6 - текущий параметр
	if p6 != 0 then
    S_chan param_chan p4, p6
    cabbageSet sprintf("%s-bg", S_chan), "colour", 56, 83, 74, 80
    cabbageSet sprintf("%s-bg", S_chan), "outlineThickness", 0
	endif

	if p5 != 0 then
    S_chan param_chan p4, p5
    cabbageSet sprintf("%s-bg", S_chan), "colour", 56, 83, 74, 90
    cabbageSet sprintf("%s-bg", S_chan), "outlineThickness", 20
	endif

  cabbageSetValue "gkParam", p5
  cabbageSet "gkParam-value", sprintf({{text("%d")}}, p5)
  turnoff
endin

; изминить значение параметра на величину инкримента
instr edit, 103
  ; p4 - номер голоса
  ; p5 - MIDI CC, индекс параметра giP
  ; p6 - значение скорости

	istep, i_new_value, imin, imax, idef init 0
  vtabi p5, giP, istep, imin, imax, idef

  ; канал параметра
  S_param param_chan p4, p5

  i_curr_value = cabbageGetValue:i(S_param)
	i_step_max init imax * 0.1
	i_step_min init 0.00001

	if p6 == 2 || p6 == 7 then
    istep = limit(0.25 * istep, i_step_min, i_step_max)
	elseif p6 == 3 || p6 == 8 then
    istep = limit(0.66 * istep, i_step_min, i_step_max)
	elseif p6 == 4 || p6 == 9 then
    istep = limit(istep + 0.25 * istep, i_step_min, i_step_max)
	elseif p6 == 5 || p6 == 10 then
    istep = limit(istep + 0.66 * istep, i_step_min, i_step_max)
  endif

	if p6 < 6 then
		istep = -istep
  endif

  i_new_value = limit(i_curr_value + istep, imin, imax)

  if i_new_value != i_curr_value then
    cabbageSetValue S_param, i_new_value
  endif
endin


; всё MIDI пропускать через router
massign -1, -1
massign 0, "midi_router"
; and programs are assigned to test instr
pgmassign 0, "midi_router"
; routing пришедших MIDI сообщений
instr midi_router, 100
  ; kcode - тип MIDI собщения
  ; kch  - MIDI Ch, индекс голоса
  ; kidx - индекс MIDI собщения: нота, CC
  ; kval - значение MIDI собщения: громкость ноты, значение CC
  kcode, kch, kidx, kval init 0

  kcode, kch, kidx, kval midiin

  kLayout = chnget:k("gkLayout")
  kParam = chnget:k("gkParam")
  kChan = chnget:k("gkChan")

  ; ksig miditempo
  ; prints "miditempo %d ", ksig

  ; midi status code
  ;     0 - no midi or pending
	;   144 - midi note
	;   176 - midi CC
  ;   160 - aftertouch
  ;   208 - channel aftertouch
  ; ckgoto (kcode == 0 || kcode == 160 || kcode == 208), THEEND

  ; 1 - midi note command NOTE COMMAND->LOAD_LAYOUT
  if kcode == 144 && kidx == 1 then
    cabbageSetValue tab_chan(kval), 1, (kLayout == kval ? 0 : 1)

  ; 2 - midi note command NOTE COMMAND->EDIT_PARAM
  elseif kcode == 144 && kidx == 2 && kParam != kval && kval != 0 then
    event "i", "set_param", 0, 1, kch-1, kval, kParam

  ; MIDI CC->EDIT VALUE
  elseif kcode == 176 && kval > 0 then
    event "i", "edit", 0, 1, kch-1, kidx, kval

  endif

  ; k-time, не чаще раза в 0.5 секунды
  ; printks " trig next layout = %d\n", 0.5, kval
  ; THEEND:
  ; обработка MIDI сообщения завершена
endin
; запустить миди роутер
turnon 100


</CsInstruments>
<CsScore>
i "voices"      0  z  4
i "gui_event"  +1  z

; i "borovylo"       0 -1 1 1
</CsScore>

</CsoundSynthesizer>
