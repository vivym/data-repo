import { GithubOutlined } from '@ant-design/icons';
import { DefaultFooter } from '@ant-design/pro-components';
import React from 'react';

const Footer: React.FC = () => {
  return (
    <DefaultFooter
      style={{
        background: 'none',
      }}
      links={[
        {
          key: 'github',
          title: <GithubOutlined />,
          href: 'https://github.com/vivym/data-repo',
          blankTarget: true,
        },
        {
          key: 'OmniAI',
          title: 'OmniAI',
          href: 'https://omniai.cc',
          blankTarget: true,
        },
      ]}
    />
  );
};

export default Footer;
