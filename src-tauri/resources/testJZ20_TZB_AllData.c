
#include "JZ20_TZB_AllData.h"
#include <gtest/gtest.h>
#include "JZ20_TZB_CF.h"


TEST(testJZ20_TZB_AllData, test_Safe_temp_Init_n_initializes_specific_element) {
  uint8_t num = 2;
  Safe_temp_Init_n(num);
  ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);
}


TEST(testJZ20_TZB_AllData, test_Safe_temp_Init_initializes_all_elements) {
  Safe_temp_Init();
  ASSERT_EQ(Recv_SafeData[0].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[0].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[0].Fill_Len, 0U);
  ASSERT_EQ(Recv_SafeData[1].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[1].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[1].Fill_Len, 0U);
  ASSERT_EQ(Recv_SafeData[2].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[2].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[2].Fill_Len, 0U);
  ASSERT_EQ(Recv_SafeData[3].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[3].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[3].Fill_Len, 0U);
}

TEST(testJZ20_TZB_AllData, test_Safe_temp_Init_n_initializes_specific_element) {
  uint8_t num = 1;
  Safe_temp_Init_n(num);
  ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);
}



TEST(testJZ20_TZB_AllData, test_htonl_t_big_little_end_not_0x55) {
  rssp_cfg_info.big_little_end = 0xAA;
  uint32_t input = 0x12345678;
  ASSERT_EQ(htonl_t(input), input);
}


TEST(testJZ20_TZB_AllData, test_htonl_t_big_little_end_0x55) {
  rssp_cfg_info.big_little_end = 0x55;
  uint32_t input = 0x12345678;
  uint32_t expected_output = 0x78563412;
  ASSERT_EQ(htonl_t(input), expected_output);
}


TEST(testJZ20_TZB_AllData, test_Safe_temp_Init_n_initializes_specific_element_different_element) {
  uint8_t num = 3;
  Safe_temp_Init_n(num);
  ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);
}



TEST(testJZ20_TZB_AllData, test_Safe_temp_Init_n_initializes_specific_element_different_element) {
  uint8_t num = 3;
  Safe_temp_Init_n(num);
  ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
  ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
  ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);
}

uint8_t num = 4;
Safe_temp_Init_n(num);
ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);



uint8_t num = 4;
Safe_temp_Init_n(num);
ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);


uint8_t num = 4;
Safe_temp_Init_n(num);
ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);


uint8_t num = 4;
Safe_temp_Init_n(num);
ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);


uint8_t num = 4;
Safe_temp_Init_n(num);
ASSERT_EQ(Recv_SafeData[num].FindHead, 0U);
ASSERT_EQ(Recv_SafeData[num].Data_Len, 0U);
ASSERT_EQ(Recv_SafeData[num].Fill_Len, 0U);






























