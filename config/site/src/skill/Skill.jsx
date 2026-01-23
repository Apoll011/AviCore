import React, { useState, useEffect } from 'react';
import {Settings, Package, Code, Shield, Zap, FileText, Activity } from 'lucide-react';
import SettingsPage from "./SettingsPage.jsx";
import Overview from "./Overview.jsx";

const fetchSkillData = async () => {
    return {
        skill: {
            id: "light_control",
            name: "Light Control",
            description: "A skill to control the lights",
            entry: "main.avi",
            capabilities: ["intent:light.turn_on", "intent:light.turn_off"],
            permissions: ["speak", "display"],
            author: "Avi Labs",
            version: "1.0.0"
        }
    };
};

const Skill = () => {
    const [data, setData] = useState({ skill: null });
    const [activeTab, setActiveTab] = useState('overview');

    const loadData = async () => {
        const result = await fetchSkillData();
        setData(result);
    };

    useEffect(() => {
        loadData();
    }, []);

    const { skill } = data;

    const tabs = [
        { id: 'overview', label: 'Overview', icon: FileText },
        { id: 'configuration', label: 'Configuration', icon: Settings },
        { id: 'analytics', label: 'Analytics', icon: Activity },
    ];

    return (
        <div className="min-h-screen bg-gray-950 text-white flex">
            {/* Sidebar */}
            <div className="w-80 bg-gray-900 border-r border-gray-800 flex flex-col">
                {/* Sidebar Header */}
                <div className="p-4 border-b border-gray-800">
                    <div className="flex items-center gap-2">
                        <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                            <Package size={18} className="text-white" />
                        </div>
                        <div>
                            <h3 className="text-sm font-semibold">Skills</h3>
                            <p className="text-xs text-gray-500">Configuration</p>
                        </div>
                    </div>
                </div>

                {/* Skill Tree */}
                {skill && (
                    <div className="flex-1 overflow-y-auto p-3">
                        <div className="mb-4">
                            <p className="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2 px-2">Skill</p>
                            <div className="bg-gray-800/50 rounded-lg p-3 border border-gray-700/50 hover:border-gray-600 transition-colors cursor-pointer">
                                <div className="flex items-center gap-2 mb-2">
                                    <Package size={16} className="text-blue-400" />
                                    <span className="text-sm font-medium">{skill.name}</span>
                                </div>
                                <p className="text-xs text-gray-400 mb-2">{skill.description}</p>
                                <div className="flex items-center justify-between">
                                    <span className="text-xs text-gray-500">v{skill.version}</span>
                                    <span className="text-xs text-gray-500">{skill.author}</span>
                                </div>
                            </div>
                        </div>

                        <div className="space-y-1">
                            <p className="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2 px-2">Properties</p>

                            <div className="px-2 py-1.5 hover:bg-gray-800 rounded cursor-pointer transition-colors">
                                <div className="flex items-center gap-2">
                                    <Code size={14} className="text-gray-400" />
                                    <span className="text-xs text-gray-300">Entry Point</span>
                                </div>
                                <p className="text-xs text-gray-500 ml-5 font-mono">{skill.entry}</p>
                            </div>

                            <div className="px-2 py-1.5 hover:bg-gray-800 rounded cursor-pointer transition-colors">
                                <div className="flex items-center gap-2">
                                    <Zap size={14} className="text-gray-400" />
                                    <span className="text-xs text-gray-300">Capabilities</span>
                                    <span className="ml-auto text-xs text-gray-500">{skill.capabilities?.length || 0}</span>
                                </div>
                            </div>

                            <div className="px-2 py-1.5 hover:bg-gray-800 rounded cursor-pointer transition-colors">
                                <div className="flex items-center gap-2">
                                    <Shield size={14} className="text-gray-400" />
                                    <span className="text-xs text-gray-300">Permissions</span>
                                    <span className="ml-auto text-xs text-gray-500">{skill.permissions?.length || 0}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                )}
            </div>

            {/* Main Content */}
            <div className="flex-1 flex flex-col">
                {/* Tab Bar */}
                <div className="bg-gray-900 border-b border-gray-800 flex items-center px-4">
                    {tabs.map((tab) => {
                        const Icon = tab.icon;
                        return (
                            <button
                                key={tab.id}
                                onClick={() => setActiveTab(tab.id)}
                                className={`flex items-center gap-2 px-4 py-3 text-sm border-b-2 transition-colors ${
                                    activeTab === tab.id
                                        ? 'border-blue-500 text-white bg-gray-800/50'
                                        : 'border-transparent text-gray-400 hover:text-gray-300 hover:bg-gray-800/30'
                                }`}
                            >
                                <Icon size={16} />
                                {tab.label}
                            </button>
                        );
                    })}
                </div>

                {/* Content Area */}
                <div className="flex-1 overflow-y-auto bg-gray-950">
                    {skill && activeTab === 'overview' && (
                        <Overview skill={skill}/>
                    )}

                    {activeTab === 'configuration' && (
                        <div className="p-8">
                            <h2 className="text-2xl font-bold mb-6">Configuration Settings</h2>
                            <SettingsPage />
                        </div>
                    )}

                    {activeTab === 'analytics' && (
                        <div className="p-8">
                            <h2 className="text-2xl font-bold mb-6">Analytics Dashboard</h2>
                            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                <div className="bg-gray-900 rounded-lg p-6 border border-gray-800">
                                    <p className="text-gray-400 text-sm mb-2">Total Invocations</p>
                                    <p className="text-3xl font-bold">1,247</p>
                                </div>
                                <div className="bg-gray-900 rounded-lg p-6 border border-gray-800">
                                    <p className="text-gray-400 text-sm mb-2">Success Rate</p>
                                    <p className="text-3xl font-bold">98.4%</p>
                                </div>
                                <div className="bg-gray-900 rounded-lg p-6 border border-gray-800">
                                    <p className="text-gray-400 text-sm mb-2">Avg Response Time</p>
                                    <p className="text-3xl font-bold">142ms</p>
                                </div>
                            </div>
                        </div>
                    )}

                </div>
            </div>
        </div>
    );
};

export default Skill;