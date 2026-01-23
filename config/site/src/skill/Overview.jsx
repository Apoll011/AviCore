import React from 'react';
import { Package, User, Shield, Zap } from 'lucide-react';

const Overview = ( {skill} ) => {
    return (
        <div className="p-8">
            <h1 className="text-3xl font-bold mb-2">{skill.name}</h1>
            <p className="text-gray-400 mb-8">{skill.description}</p>

            {/* Stats Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
                <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-1">
                        <User size={16} />
                        <span className="text-xs">Author</span>
                    </div>
                    <p className="text-lg font-semibold">{skill.author}</p>
                </div>
                <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-1">
                        <Package size={16} />
                        <span className="text-xs">Version</span>
                    </div>
                    <p className="text-lg font-semibold">v{skill.version}</p>
                </div>
                <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-1">
                        <Zap size={16} />
                        <span className="text-xs">Capabilities</span>
                    </div>
                    <p className="text-lg font-semibold">{skill.capabilities?.length || 0}</p>
                </div>
                <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-1">
                        <Shield size={16} />
                        <span className="text-xs">Permissions</span>
                    </div>
                    <p className="text-lg font-semibold">{skill.permissions?.length || 0}</p>
                </div>
            </div>
            {/* Capabilities Section */}
            {skill.capabilities && skill.capabilities.length > 0 && (
                <div className="mb-6">
                    <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">Capabilities</h3>
                    <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                        <div className="space-y-2">
                            {skill.capabilities.map((cap, i) => (
                                <div key={i} className="flex items-center gap-2 py-2 px-3 bg-gray-800/50 rounded border border-gray-700/50">
                                    <Zap size={14} className="text-blue-400" />
                                    <code className="text-sm text-gray-300 font-mono">{cap}</code>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            )}
            {/* Permissions Section */}
            {skill.permissions && skill.permissions.length > 0 && (
                <div>
                    <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">Permissions</h3>
                    <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
                        <div className="space-y-2">
                            {skill.permissions.map((perm, i) => (
                                <div key={i} className="flex items-center gap-2 py-2 px-3 bg-gray-800/50 rounded border border-gray-700/50">
                                    <Shield size={14} className="text-green-400" />
                                    <code className="text-sm text-gray-300 font-mono">{perm}</code>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default Overview;