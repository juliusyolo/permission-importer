/*
 Navicat Premium Data Transfer

 Source Server         : julius_mysql
 Source Server Type    : MySQL
 Source Server Version : 80027 (8.0.27)
 Source Host           : 127.0.0.1:3306
 Source Schema         : permission_control

 Target Server Type    : MySQL
 Target Server Version : 80027 (8.0.27)
 File Encoding         : 65001

 Date: 15/06/2023 18:03:12
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for version_control
-- ----------------------------
DROP TABLE IF EXISTS `version_control`;
CREATE TABLE `version_control` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `version` varchar(255) COLLATE utf8mb4_bin NOT NULL COMMENT '版本号',
  `release_time` datetime NOT NULL COMMENT '上线时间',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_version` (`version`) USING BTREE COMMENT '版本号唯一索引'
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

-- ----------------------------
-- Records of version_control
-- ----------------------------
BEGIN;
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (1, 'BH_RCMP.PLS_23.05.01', '2023-06-14 10:05:41', '2023-06-14 02:05:48', '2023-06-14 02:06:15');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (2, 'BH_RCMP.PLS_23.05.02', '2023-06-14 10:06:24', '2023-06-14 02:06:27', '2023-06-14 02:06:27');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (4, 'BH_RCMP.PLS_23.05.03', '2023-06-14 14:15:58', '2023-06-14 06:15:41', '2023-06-14 06:16:07');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (5, 'BH_RCMP.PLS_23.05.04', '2023-06-14 14:15:54', '2023-06-14 06:15:57', '2023-06-14 06:15:57');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (6, 'BH_RCMP.PLS_23.05.05', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (7, 'BH_RCMP.PLS_23.05.06', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (8, 'BH_RCMP.PLS_23.05.07', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (9, 'BH_RCMP.PLS_23.05.08', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (10, 'BH_RCMP.PLS_23.05.09', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (11, 'BH_RCMP.PLS_23.05.10', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (12, 'BH_RCMP.PLS_23.05.11', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (13, 'BH_RCMP.PLS_23.05.12', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (14, 'BH_RCMP.PLS_23.05.13', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (15, 'BH_RCMP.PLS_23.05.14', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (16, 'BH_RCMP.PLS_23.05.15', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (17, 'BH_RCMP.PLS_23.05.16', '2023-06-14 10:05:41', '2023-06-14 06:18:28', '2023-06-14 06:18:28');
INSERT INTO `version_control` (`id`, `version`, `release_time`, `create_time`, `update_time`) VALUES (20, 'BH_RCMP.PLS_23.05.17', '2023-06-15 16:18:01', '2023-06-15 08:18:05', '2023-06-15 08:18:05');
COMMIT;

SET FOREIGN_KEY_CHECKS = 1;
