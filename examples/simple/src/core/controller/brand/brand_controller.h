// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/create_brand_dto.h"
#include "brand/update_brand_dto.h"
#include "event_dispatcher.h"
#include "repository/interface_repository_provider.h"
#include "simple_example_controller_export.h"

#include "undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>

using namespace Simple::Contracts::Repository;
using namespace Simple::Controller::UndoRedo;
using namespace Simple::Contracts::DTO::Brand;

namespace Simple::Controller::Brand
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT BrandController : public QObject
{
    Q_OBJECT
  public:
    explicit BrandController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                             QSharedPointer<EventDispatcher> eventDispatcher);

    static BrandController *instance();

    Q_INVOKABLE QCoro::Task<BrandDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<QList<BrandDTO>> getAll() const;

    Q_INVOKABLE static Contracts::DTO::Brand::CreateBrandDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Brand::UpdateBrandDTO getUpdateDTO();

  public Q_SLOTS:

    QCoro::Task<BrandDTO> create(const CreateBrandDTO &dto);

    QCoro::Task<BrandDTO> update(const UpdateBrandDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<BrandController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    BrandController() = delete;
    BrandController(const BrandController &) = delete;
    BrandController &operator=(const BrandController &) = delete;
};

} // namespace Simple::Controller::Brand