import {
  ModalForm,
  ProFormText,
} from '@ant-design/pro-components';
import { useIntl } from '@umijs/max';
import React from 'react';

export type FormValueType = {
  password?: string;
} & Partial<API.User>;

export type UpdateFormProps = {
  onCancel: (flag?: boolean, formVals?: FormValueType) => void;
  onSubmit: (values: FormValueType) => Promise<void>;
  updateModalOpen: boolean;
  values: Partial<API.User>;
};

const UpdateForm: React.FC<UpdateFormProps> = (props) => {
  const intl = useIntl();
  return (
    <ModalForm
      title="修改用户信息"
      initialValues={{
        nickname: props.values.nickname,
        avatarUri: props.values.avatar_uri,
      }}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
        onCancel: () => {
          props.onCancel();
        },
      }}
      open={props.updateModalOpen}
      onFinish={props.onSubmit}
    >
      <ProFormText
        width="lg"
        name="password"
        label={intl.formatMessage({
          id: 'pages.userList.colums.password',
          defaultMessage: 'Password',
        })}
        tooltip={intl.formatMessage({
          id: 'pages.userList.colums.password.tooltip',
          defaultMessage: 'At least 3 characters',
        })}
        placeholder={intl.formatMessage({
          id: 'pages.userList.colums.password.placeholder',
          defaultMessage: 'Please enter the password',
        })}
      />
      <ProFormText
        width="lg"
        name="nickname"
        label={intl.formatMessage({
          id: 'pages.userList.colums.nickname',
          defaultMessage: '昵称',
        })}
        tooltip={intl.formatMessage({
          id: 'pages.userList.colums.nickname.tooltip',
          defaultMessage: 'At least 3 characters',
        })}
        placeholder={intl.formatMessage({
          id: 'pages.userList.colums.nickname.placeholder',
          defaultMessage: '请输入用户昵称',
        })}
      />
      <ProFormText
        width="lg"
        name="avatarUri"
        label={intl.formatMessage({
          id: 'pages.userList.colums.avatarUri',
          defaultMessage: 'Avatar URL',
        })}
        placeholder={intl.formatMessage({
          id: 'pages.userList.colums.avatarUri.placeholder',
          defaultMessage: 'Please enter the avatar URL',
        })}
      />
    </ModalForm>
  );
};

export default UpdateForm;
