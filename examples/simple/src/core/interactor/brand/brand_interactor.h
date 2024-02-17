// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/create_brand_dto.h"
#include "brand/update_brand_dto.h"
#include "event_dispatcher.h"
#include "simple_example_interactor_export.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Simple::Contracts::DTO::Brand;

namespace Simple::Interactor::Brand
{

class SIMPLE_EXAMPLE_INTERACTOR_EXPORT BrandInteractor : public QObject
{
    Q_OBJECT
  public:
    explicit BrandInteractor(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                             QSharedPointer<EventDispatcher> eventDispatcher);

    static BrandInteractor *instance();

    Q_INVOKABLE QCoro::Task<BrandDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<QList<BrandDTO>> getAll() const;

    Q_INVOKABLE static Contracts::DTO::Brand::CreateBrandDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Brand::UpdateBrandDTO getUpdateDTO();

  public slots:

    QCoro::Task<BrandDTO> create(const CreateBrandDTO &dto);

    QCoro::Task<BrandDTO> update(const UpdateBrandDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<BrandInteractor> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    BrandInteractor() = delete;
    BrandInteractor(const BrandInteractor &) = delete;
    BrandInteractor &operator=(const BrandInteractor &) = delete;
};

} // namespace Simple::Interactor::Brand