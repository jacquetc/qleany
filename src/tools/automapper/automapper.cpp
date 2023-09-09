// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#include "qleany/tools/automapper/automapper.h"

// QHash<QMetaType, std::function<QVariant(const QVariant&)>>
// AutoMapper::AutoMapper::conversions;
QHash<QMetaType, std::function<QVariant()>> Qleany::Tools::AutoMapper::AutoMapper::getDefaultSiblingFunctions;
QHash<QPair<QMetaType, QMetaType>, std::function<QVariant()>>
    Qleany::Tools::AutoMapper::AutoMapper::getSiblingFunctions;

QHash<QPair<QMetaType, QMetaType>,
      std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer, const QVariant &sourceValue)>>
    Qleany::Tools::AutoMapper::AutoMapper::writerHash;

QHash<QPair<QMetaType, QMetaType>, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer,
                                                      const QList<QVariant> &sourceList)>>
    Qleany::Tools::AutoMapper::AutoMapper::writerForListHash;
QMutex Qleany::Tools::AutoMapper::AutoMapper::mutex;
