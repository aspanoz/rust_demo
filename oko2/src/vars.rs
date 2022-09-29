#![allow(dead_code)]
#![allow(unused_variables)]
/// Этот файл автоматически создан скриптом build.rs

/// Версия приложения, данные из ./Cargo.toml
pub static VERSION: &'static str = "0.1.4";

/// Название исполняемого файла, данные из ./Cargo.toml
pub static APP_ID: &'static str = "podula";

/// Описание приложения, данные из ./Cargo.toml
pub static DESCRIPTION: &'static str = "Popedula - миди контроллер";

/// Название приеложения, данные из ./Cargo.toml
pub static LABEL: &'static str = "Popedula";

/// Порт WebSocket сервера
pub static PORT: u16 = 5000;

/// csound
pub static ORC: &'static str = r##"; jack_control dps rate 48000
sr = 48000
; jack_control dps period 256
ksmps = 256
nchnls = 2
0dbfs = 1
nchnls_i = 0

massign -1, -1
massign 0, "midi_router"
alwayson "midi_router"

git init 2  ; кол-во трэков
gip init 21 ; кол-во параметров

gi_params ftgen 0, 0, gip, -7, 0 ; table 101, значения параметров

;                            размер       генератор
;                            |            |
gi_Attdec      ftgen 0, 0,   524288,      19, 0.5, 0.5, 270, 0.5
gi_Triangle    ftgen 0, 0,   4097,        20, 3
gi_tracks      ftgen 0, 0,   git,         -7, 0 ; готовность трэков (загружен ли сэмпл)

vtabwi 0,  gi_params, 0.3   ; "level"
vtabwi 1,  gi_params, 30.0  ; "density"
vtabwi 2,  gi_params, 0.1   ; "size"
vtabwi 3,  gi_params, 0.0   ; "oct_div"
vtabwi 4,  gi_params, 0.0   ; "freeze"
vtabwi 5,  gi_params, 1.0   ; "pitch"
vtabwi 6,  gi_params, 0.0   ; "port"
vtabwi 7,  gi_params, 0.0   ; "rnd_pos"
vtabwi 8,  gi_params, 0.0   ; "rnd_pitch"
vtabwi 9,  gi_params, 5.0   ; "band"
vtabwi 10, gi_params, 1.0   ; "lfo_rate"
vtabwi 11, gi_params, 0.0   ; "lfo_dens"
vtabwi 12, gi_params, 0.0   ; "lfo_size"
vtabwi 13, gi_params, 1.0   ; "speed"
vtabwi 14, gi_params, 0.0   ; "start"
vtabwi 15, gi_params, 1.0   ; "range"
vtabwi 16, gi_params, 0.1   ; "release"
vtabwi 17, gi_params, 0.005 ; "decay"
vtabwi 18, gi_params, 0.002 ; "rise"
vtabwi 19, gi_params, 0.005 ; "attack"
vtabwi 20, gi_params, 0.0   ; "rnd_dens"

; кусок пути используется при инициализации в случае загрузки сэмпла
gSRoot init "/home/const/.config/borovylo"
gSProject init "nikozz2022"
gSMedia init ""
gSMedia chnexport "sample_file", 1

; init
gk_shape_1 init 1
gk_mode_1 init 1
;                                     read & wright flag
;                                     |   integer - 1, linear - 2, exponential - 3
;                                     |   |   default          min    max
gk_shape_1   chnexport "shape_1",     3,  1,  i(gk_shape_1),   1,     3
gk_mode_1    chnexport "mode_1",      3,  1,  i(gk_mode_1),    1,     2

; рассчитать размер таблицы для сэмпла
opcode calculate_sample_table_size, i, i
  i_val xin
  i_count init 1
  i_result init 2
  until i_result >= i_val do
    i_count += 1
    i_result = 2^i_count
  od
  xout i_result
endop

; загрузка сэмпла
instr 91, sample_loader
  iN = p3
  i_offset = p4
  i_range = p5
  iL init 2 * iN - 1
  iR init 2 * iN

  ; printf "[sample_loader] %s \n", (strlen(gSMedia) == 0 ? 1 : 0), "empty file name"
  ckgoto strlen(gSMedia) == 0, THEEND
  S_filepath init sprintf("%s/%s/%s", gSRoot, gSProject, gSMedia)
  ; printf "[sample_loader] invalid file '%s'\n", (filevalid(S_filepath) == 0 ? 1 : 0), S_filepath
  ckgoto filevalid(S_filepath) == 0, THEEND

  ; установить треку состояние "файл не загружен"
  vtabwi iN - 1, gi_tracks, 0

  ; сбросить таблцу с загруженым сэмплом
  if ftexists(iR) == 1 then
    ftfree iL, 0
    ftfree iR, 0
  elseif ftexists(iL) == 1 then
    ftfree iL, 0
  endif

  i_tablen_sec = filelen:i(S_filepath)
  if i_offset > 0 || i_range != 1 then
    ; вычисление длительности сэмпла
    i_tablen_sec = (i_tablen_sec - i_offset) * i_range
  endif

  i_ftlen calculate_sample_table_size i_tablen_sec * sr

  iftn ftgen iL, 0, i_ftlen, 1, S_filepath, i_offset, 0, 1

  i_stereo = filenchnls(S_filepath)
  if i_stereo == 2 then
    iftn2 ftgen iR, 0, i_ftlen, 1, S_filepath, i_offset, 0, 2
  endif

  ; после загрузки файла установить флаг готовности трэка
  vtabwi iN - 1, gi_tracks, i_stereo

  THEEND:
    gSMedia = ""
    turnoff
endin


opcode get_envelope, a, kk
  i_attack, i_release xin
  if i_attack > 0 then
    k_envelope linsegr 0, i_attack, 1, i_release, 0
  else
    k_envelope linsegr 1, i_release, 0
  endif
  k_envelope expcurve k_envelope, 8
  a_envelope interp k_envelope
  xout a_envelope
endop


; вычисление положения позиции проигывания для fog
opcode get_playhead_pos, a, kkkkkkii
    k_Pmd, k_Phs, k_Shape, k_Speed, \
    k_Freeze, k_Range, i_sample_table_id, i_Triangle \
  xin
  k_pmd_rnd unirand k_Pmd
  ; printf "\n !! i_sample_cnst = %f\n", 1, k_Range

  k_porttime linseg 0, 0.001, 0.05
  i_sample_cnst = nsamp(i_sample_table_id)/ftlen(i_sample_table_id)
  k_freeze_range_sample = (k_Speed * sr * (1 - k_Freeze)) / (nsamp(i_sample_table_id) * k_Range)

  ;   k_Shape: 1 - phasor
  if k_Shape == 1 then
    k_ptr phasor k_freeze_range_sample

  ;   k_Shape: 2 - tri.
  elseif k_Shape == 2 then
    k_ptr oscili 1, k_freeze_range_sample, i_Triangle

  ;   k_Shape: 3 - sine
  elseif k_Shape == 3 then
    k_ptr oscili 0.5, k_freeze_range_sample
    k_ptr += 0.5
  endif

  k_ptr = k_ptr * i_sample_cnst * k_Range
  k_ptr += (k_Phs + k_pmd_rnd) * i_sample_cnst
  k_ptr mirror k_ptr, 0, i_sample_cnst

  a_ptr interp k_ptr
  xout a_ptr
endop


instr 93, borovylo
  iN = p4
  i_note = p5
  i_stereo tab_i iN - 1, gi_tracks
  ; printks2 sprintfk("** on midi channel %d, note %d\n", iN, i_note), changed2(i_note)

  mididefault 44, iN

  k_level init 0
  k_dens init 30.0
  k_size init 0.1
  k_oct_div init 0
  k_freeze init 0
  k_pitch init 1
  k_port init 0
  k_rnd_playhead init 0
  k_rnd_pitch init 0
  k_band init 5
  k_lfo_dens init 0
  k_lfo_size init 0
  k_lfo_rate init 0
  k_speed init 1
  k_start init 0
  k_range init 1
  k_release init 0.1
  k_decay init 0.005
  k_rise init 0.002
  k_attack init 0.005
  k_rnd_dens init 0

  vtabk 0, gi_params, k_level
  vtabk 1, gi_params, k_dens
  vtabk 2, gi_params, k_size
  vtabk 3, gi_params, k_oct_div
  vtabk 4, gi_params, k_freeze
  vtabk 5, gi_params, k_pitch
  vtabk 6, gi_params, k_port
  vtabk 7, gi_params, k_rnd_playhead
  vtabk 8, gi_params, k_rnd_pitch
  vtabk 9, gi_params, k_band
  vtabk 11, gi_params, k_lfo_dens
  vtabk 12, gi_params, k_lfo_size
  vtabk 10, gi_params, k_lfo_rate
  vtabk 13, gi_params, k_speed
  vtabk 14, gi_params, k_start
  vtabk 15, gi_params, k_range
  vtabk 16, gi_params, k_release
  vtabk 17, gi_params, k_decay
  vtabk 18, gi_params, k_rise
  vtabk 19, gi_params, k_attack
  vtabk 20, gi_params, k_rnd_dens

  k_shape chnget sprintf("shape_%d", iN)
  k_mode chnget sprintf("mode_%d", iN)

  if changed2(k_mode) == 1 then
    reinit START
  endif

  i_frq_ratio = i_note/44 ; 60 - С3
  i_sample_left = 2*iN - 1
  i_sample_right = 2*iN

  ; ENVELOPE
  START:
    a_envelope get_envelope k_attack, k_release

    a_ptr get_playhead_pos k_rnd_playhead, k_start, k_shape, k_speed, \
                           k_freeze, k_range, i_sample_left, gi_Triangle

    ; k_level *= iamp

    ; RANDOMIZE
    kPchRnd bexprnd k_rnd_pitch
    kpch = i_frq_ratio * k_pitch * octave(kPchRnd)

    kRndTrig init 1
    kDensRnd bexprnd k_rnd_dens
    kdens = k_dens * octave(kDensRnd)
    kRndTrig metro kdens

    ; LFO
    kDensLFO poscil k_lfo_dens, k_lfo_rate
    kdens = kdens * octave(kDensLFO)

    kSizeLFO poscil k_lfo_size, k_lfo_rate
    kdur = k_size * octave(kSizeLFO)

    ; @WUT?
    iNumOverLaps = 3000
    itotdur = 3600

    a_left fog k_level, kdens, kpch, a_ptr, \
               k_oct_div, k_band, k_rise, kdur, \
               k_decay, iNumOverLaps, i_sample_left, gi_Attdec, \
               itotdur, 0, i(k_mode), 1
    a_left = a_left * a_envelope

    if i_stereo == 2 then
      a_right fog k_level, kdens, kpch, a_ptr, \
                  k_oct_div, k_band, k_rise, kdur, \
                  k_decay, iNumOverLaps, i_sample_right, gi_Attdec, \
                  itotdur, 0, i(k_mode), 1
      a_right = a_right * a_envelope
    else
      a_right = a_left
    endif

  ; выкидывать стереo
  outs a_left, a_right

  rireturn
endin


instr 90, midi_router
  kcode, kch, kidx, kval init 0
  ; setksmps 1 ; перекрываю ksmps - the number of samples in a control period
  kcode, kch, kidx, kval midiin

  ; midi status code
  ;   0 - no midi or pending
  ; 160 - aftertouch
  ; 208 - channel aftertouch
  ckgoto (kcode == 0 || kcode == 160 || kcode == 208 || kch == 0), THEEND

  ; printsk sprintfk("borovylo off %d, note %d, velocty %d\n", kch, kidx, kval)

  ;
  ; РЕЖИМ БОРОВЫЛА, note on/off
  ;
  ; midi status code
  ; 128 - midi note off
  ; 144 - midi note on
  ; if kcode == 144 && kstate == 0 then
  if kcode == 144 then
    ; если midi note on
    event "i", 93.0 + 0.01 * kch * kidx, 0, 1200, kch, kidx
    kgoto THEEND
  endif

  if kcode == 128 then
    ; если midi note off
    turnoff2 93.0 + 0.01 * kch * kidx, 4, 1
  endif

  THEEND:
endin
"##;
